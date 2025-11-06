#!/usr/bin/env perl
# Reverse tracing migration for CLI files - they need stdout, not logs
#
# Usage:
#   ./scripts/oneoffs/20251105-unmigrate-cli-from-tracing.pl [--dry-run]
#
# This script reverses the tracing migration for CLI files:
#   info!(...) → println!(...)
#   error!(...) → eprintln!(...)
#   warn!(...) → eprintln!(...)
#   debug!(...) → println!(...)  (rare in CLI, but keep as println)
#
# Strategy:
# 1. Find all CLI files in src/cli/
# 2. Replace tracing macros with println!/eprintln!
# 3. Remove tracing use statements from CLI files

use strict;
use warnings;
use File::Find;
use File::Slurp qw(read_file write_file);
use Getopt::Long;

my $dry_run = 0;
GetOptions('dry-run' => \$dry_run) or die "Usage: $0 [--dry-run]\n";

print "=== Reverse Tracing Migration for CLI ===\n";
print "Mode: " . ($dry_run ? "DRY RUN (no changes)" : "LIVE (will modify files)") . "\n";
print "\n";

# Find all CLI files
my @files;
find(sub {
    return unless -f $_ && /\.rs$/;
    push @files, $File::Find::name;
}, 'src/cli');

my %stats = (
    total_files => 0,
    modified_files => 0,
    total_replacements => 0,
    by_type => {},
);

for my $file (sort @files) {
    my $content = read_file($file);
    my $original = $content;
    my $file_modified = 0;
    my %file_replacements;

    # Find all tracing macro calls (info!, error!, warn!, debug!)
    my @matches;
    while ($content =~ /((?:info|error|warn|debug)!\s*\([^;]*;)/g) {
        push @matches, $1;
    }

    next unless @matches;

    $stats{total_files}++;

    print "Processing: $file\n";
    print "  Found " . scalar(@matches) . " tracing statement(s)\n";

    # Replace each tracing macro
    for my $match (@matches) {
        my $old = $match;
        my $new = $match;
        my $macro_type;

        if ($new =~ /^info!/) {
            $new =~ s/^info!/println!/;
            $macro_type = 'info→println';
        } elsif ($new =~ /^error!/) {
            $new =~ s/^error!/eprintln!/;
            $macro_type = 'error→eprintln';
        } elsif ($new =~ /^warn!/) {
            $new =~ s/^warn!/eprintln!/;
            $macro_type = 'warn→eprintln';
        } elsif ($new =~ /^debug!/) {
            $new =~ s/^debug!/println!/;
            $macro_type = 'debug→println';
        }

        if ($macro_type) {
            $file_replacements{$macro_type}++;
            $stats{by_type}{$macro_type}++;

            # Escape special chars and replace
            my $old_escaped = quotemeta($old);
            $content =~ s/$old_escaped/$new/;
            $file_modified = 1;
        }
    }

    # Remove tracing use statements
    if ($content =~ /use tracing::\{[^}]+\};?\n/) {
        $content =~ s/use tracing::\{[^}]+\};?\n//g;
        print "    Removed tracing use statement\n";
        $file_modified = 1;
    }
    if ($content =~ /use tracing;?\n/) {
        $content =~ s/use tracing;?\n//g;
        print "    Removed tracing use statement\n";
        $file_modified = 1;
    }

    if ($file_modified) {
        $stats{modified_files}++;
        $stats{total_replacements} += scalar(@matches);

        # Show summary for this file
        if (%file_replacements) {
            print "    Replacements: ";
            print join(", ", map { "$_=$file_replacements{$_}" } sort keys %file_replacements);
            print "\n";
        }

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
print "\nBy type:\n";
for my $type (sort keys %{$stats{by_type}}) {
    print "  $type: $stats{by_type}{$type}\n";
}
print "\n";

if ($dry_run) {
    print "DRY RUN: No files were modified.\n";
    print "Run without --dry-run to apply changes.\n";
} else {
    print "✓ Reverse migration complete!\n";
    print "\nNext steps:\n";
    print "  1. cargo build --release\n";
    print "  2. ./scripts/test.sh\n";
    print "  3. Test CLI output: cargo run --bin hegel-pm --release -- x status\n";
    print "  4. Review changes: git diff src/cli/\n";
    print "  5. Remove backups if satisfied: rm src/cli/**/*.bak\n";
    print "\nBackups saved as *.bak\n";
}
