#!/usr/bin/env perl
# Migrate println!/eprintln! to tracing macros
#
# Usage:
#   ./scripts/oneoffs/20251104-migrate-to-tracing.pl [--dry-run]
#
# This script migrates logging statements to use the tracing crate:
#   println!("🚀 ...") → info!("🚀 ...")
#   println!("✅ ...") → info!("✅ ...")
#   println!("📦 ...") → debug!("📦 ...")
#   eprintln!("❌ ...") → error!("❌ ...")
#   eprintln!("⚠️ ...") → warn!("⚠️ ...")
#
# Strategy:
# 1. Find all println!/eprintln! calls in src/ (excluding client/)
# 2. Classify by emoji prefix or keyword
# 3. Rewrite to appropriate tracing level
# 4. Add use statements where needed

use strict;
use warnings;
use File::Find;
use File::Slurp qw(read_file write_file);
use Getopt::Long;

my $dry_run = 0;
GetOptions('dry-run' => \$dry_run) or die "Usage: $0 [--dry-run]\n";

print "=== Migrate to Tracing ===\n";
print "Mode: " . ($dry_run ? "DRY RUN (no changes)" : "LIVE (will modify files)") . "\n";
print "\n";

# Classification rules (order matters - more specific first)
my @rules = (
    # Error patterns
    { pattern => qr/eprintln!\s*\([^)]*"[^"]*❌/, level => 'error', name => 'error emoji' },
    { pattern => qr/eprintln!\s*\([^)]*"[^"]*Failed/, level => 'error', name => 'Failed prefix' },
    { pattern => qr/eprintln!\s*\([^)]*"[^"]*Error:/, level => 'error', name => 'Error: prefix' },

    # Warning patterns
    { pattern => qr/eprintln!\s*\([^)]*"[^"]*⚠️/, level => 'warn', name => 'warning emoji' },
    { pattern => qr/println!\s*\([^)]*"[^"]*⚠️/, level => 'warn', name => 'warning emoji (println)' },

    # Debug patterns (cache, timing, internal details)
    { pattern => qr/println!\s*\([^)]*"[^"]*📦[^"]*[Cc]ached/, level => 'debug', name => 'cache operations' },
    { pattern => qr/println!\s*\([^)]*"[^"]*💨[^"]*[Ss]erving cached/, level => 'debug', name => 'cache hit' },
    { pattern => qr/println!\s*\([^)]*"[^"]*📊[^"]*request[^"]*completed in/, level => 'debug', name => 'request timing' },
    { pattern => qr/println!\s*\([^)]*"[^"]*📋[^"]*request[^"]*completed in/, level => 'debug', name => 'request timing' },
    { pattern => qr/println!\s*\([^)]*"[^"]*⏳[^"]*Loading/, level => 'debug', name => 'loading operations' },

    # Info patterns (general operations, success)
    { pattern => qr/println!\s*\([^)]*"[^"]*🚀/, level => 'info', name => 'startup emoji' },
    { pattern => qr/println!\s*\([^)]*"[^"]*✅/, level => 'info', name => 'success emoji' },
    { pattern => qr/println!\s*\([^)]*"[^"]*📁/, level => 'info', name => 'project operations' },
    { pattern => qr/println!\s*\([^)]*"[^"]*📍/, level => 'info', name => 'location info' },
    { pattern => qr/println!\s*\([^)]*"[^"]*🌐/, level => 'info', name => 'server operations' },
    { pattern => qr/println!\s*\([^)]*"[^"]*🌍/, level => 'info', name => 'browser operations' },
    { pattern => qr/println!\s*\([^)]*"[^"]*📝/, level => 'info', name => 'info emoji' },

    # Default: println! → info, eprintln! → error
    { pattern => qr/println!/, level => 'info', name => 'default println' },
    { pattern => qr/eprintln!/, level => 'error', name => 'default eprintln' },
);

# Files to process (exclude client/ which uses web_sys::console)
my @files;
find(sub {
    return unless -f $_ && /\.rs$/;
    return if $File::Find::name =~ m{/client/};
    return if $File::Find::name =~ m{/target/};
    push @files, $File::Find::name;
}, 'src');

my %stats = (
    total_files => 0,
    modified_files => 0,
    total_replacements => 0,
    by_level => {},
);

my @manual_cases;

for my $file (sort @files) {
    my $content = read_file($file);
    my $original = $content;
    my $file_modified = 0;
    my $needs_use_statement = 0;

    # Count matches per level for this file
    my %file_replacements;

    # Find all println!/eprintln! calls
    my @matches;
    while ($content =~ /(e?println!\s*\([^;]*;)/g) {
        push @matches, $1;
    }

    next unless @matches;

    $stats{total_files}++;

    print "Processing: $file\n";
    print "  Found " . scalar(@matches) . " logging statement(s)\n";

    # Classify and rewrite each match
    for my $match (@matches) {
        my $matched_level;
        my $matched_rule;

        # Find the first matching rule
        for my $rule (@rules) {
            if ($match =~ $rule->{pattern}) {
                $matched_level = $rule->{level};
                $matched_rule = $rule->{name};
                last;
            }
        }

        next unless $matched_level;

        # Count this replacement
        $file_replacements{$matched_level}++;
        $stats{by_level}{$matched_level}++;
        $needs_use_statement = 1;

        # Perform the replacement
        my $old = $match;
        my $new = $match;

        # Replace println!/eprintln! with tracing macro
        $new =~ s/eprintln!/${matched_level}!/;
        $new =~ s/println!/${matched_level}!/;

        # Escape regex special characters in old pattern
        my $old_escaped = quotemeta($old);

        $content =~ s/$old_escaped/$new/;
        $file_modified = 1;

        print "    $matched_rule: $matched_level\n" if $dry_run;
    }

    # Add use statement if needed and not already present
    if ($needs_use_statement && $content !~ /use tracing::/) {
        # Find a good place to add the use statement
        # Look for existing use statements
        if ($content =~ /^((?:use [^;]+;\n)+)/m) {
            # Add after existing use statements
            my $use_block = $1;
            my $new_use = "use tracing::{debug, error, info, warn};\n";
            $content =~ s/\Q$use_block\E/$use_block$new_use/;
            print "    Added use statement\n";
        } elsif ($content =~ /^(mod [^;]+;\n)/m) {
            # Add after mod statements
            my $mod_line = $1;
            $content =~ s/\Q$mod_line\E/$mod_line\nuse tracing::{debug, error, info, warn};\n/;
            print "    Added use statement after mod\n";
        } else {
            # Add at the top
            $content = "use tracing::{debug, error, info, warn};\n\n" . $content;
            print "    Added use statement at top\n";
        }
    }

    if ($file_modified) {
        $stats{modified_files}++;
        $stats{total_replacements} += scalar(@matches);

        # Show summary for this file
        print "    Replacements: ";
        print join(", ", map { "$_=$file_replacements{$_}" } sort keys %file_replacements);
        print "\n";

        unless ($dry_run) {
            # Backup original
            write_file("$file.bak", $original);
            # Write modified content
            write_file($file, $content);
            print "    ✓ File updated (backup: $file.bak)\n";
        }
    }

    print "\n";
}

# Summary
print "=== Summary ===\n";
print "Files processed:    $stats{total_files}\n";
print "Files modified:     $stats{modified_files}\n";
print "Total replacements: $stats{total_replacements}\n";
print "\nBy level:\n";
for my $level (qw(info debug warn error)) {
    my $count = $stats{by_level}{$level} // 0;
    print "  $level: $count\n";
}
print "\n";

if ($dry_run) {
    print "DRY RUN: No files were modified.\n";
    print "Run without --dry-run to apply changes.\n";
} else {
    print "✓ Migration complete!\n";
    print "\nNext steps:\n";
    print "  1. cargo build --release --features server\n";
    print "  2. cargo test --features server\n";
    print "  3. Review changes: git diff\n";
    print "  4. Test with different log levels:\n";
    print "       RUST_LOG=debug ./target/release/hegel-pm\n";
    print "       RUST_LOG=trace ./target/release/hegel-pm\n";
    print "  5. Remove backups if satisfied: rm src/**/*.bak\n";
    print "\nBackups saved as *.bak\n";
}
