#!/usr/bin/env perl
# Audit all markdown documentation in the repository
#
# Usage:
#   ./scripts/doc-audit.pl           # Default: warn about docs >1 day old
#   ./scripts/doc-audit.pl --days 7  # Warn about docs >7 days old
#
# Shows: filename, line count, and last modified date for all .md files
# Warns about potentially stale docs (excludes artifacts: .ddd/, learnings/, reports)

use strict;
use warnings;
use File::Find;
use Time::Piece;
use Getopt::Long;

# Parse arguments
my $stale_threshold_days = 1;
GetOptions('days=i' => \$stale_threshold_days) or die "Usage: $0 [--days N]\n";

print "📚 Documentation Audit\n";
print "=" x 80 . "\n\n";

# Header
printf "%-40s %10s  %s\n", "FILE", "LINES", "LAST MODIFIED";
print "-" x 80 . "\n";

# Collect all .md files
my @md_files;
find(sub {
    return unless -f $_ && /\.md$/;
    return if $File::Find::name =~ m{/node_modules/|/target/|/\.git/};
    push @md_files, $File::Find::name;
}, '.');

# Sort by path
@md_files = sort @md_files;

# Track stale docs (exclude artifacts)
my @stale_docs;
my $now = time();

for my $file (@md_files) {
    # Clean path (remove leading ./)
    my $clean_path = $file;
    $clean_path =~ s{^\./}{};

    # Get line count
    open my $fh, '<', $file or die "Can't open $file: $!";
    my $lines = 0;
    $lines++ while <$fh>;
    close $fh;

    # Get last modified time
    my $mtime = (stat($file))[9];
    my $mod_date = localtime($mtime)->strftime('%Y-%m-%d %H:%M');

    # Calculate age in days
    my $age_days = int(($now - $mtime) / 86400);

    # Check if artifact (exclude from staleness checks)
    my $is_artifact = 0;
    if ($clean_path =~ m{^\.ddd/} ||
        $clean_path =~ m{^learnings/} ||
        $clean_path eq 'COVERAGE_REPORT.md' ||
        $clean_path eq 'LOC_REPORT.md') {
        $is_artifact = 1;
    }

    # Mark as stale if over threshold and not an artifact
    my $marker = "";
    if (!$is_artifact && $age_days >= $stale_threshold_days) {
        $marker = " ⚠️";
        push @stale_docs, { path => $clean_path, days => $age_days };
    }

    printf "%-40s %10s  %s%s\n", $clean_path, $lines, $mod_date, $marker;
}

print "\n";
print "=" x 80 . "\n";

# Summary stats
my $total_files = scalar @md_files;
my $total_lines = 0;
for my $file (@md_files) {
    open my $fh, '<', $file or next;
    $total_lines++ while <$fh>;
    close $fh;
}

print "Total markdown files: $total_files\n";
print "Total lines: $total_lines\n";

# Staleness warnings
if (@stale_docs) {
    print "\n";
    print "⚠️  Potentially stale documentation (>$stale_threshold_days day" . ($stale_threshold_days == 1 ? "" : "s") . " old, excluding artifacts):\n";
    for my $doc (@stale_docs) {
        print "  • $doc->{path} ($doc->{days} days old)\n";
    }
    print "\n";
    print "Note: Artifacts (.ddd/, learnings/, generated reports) are excluded from staleness checks\n";
}

print "\n";
