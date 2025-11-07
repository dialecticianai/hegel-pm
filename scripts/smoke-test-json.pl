#!/usr/bin/env perl
use strict;
use warnings;
use Getopt::Long;

my $dry_run = 0;
my $verbose = 0;
GetOptions(
    'dry-run' => \$dry_run,
    'verbose' => \$verbose,
) or die "Usage: $0 [--dry-run] [--verbose]\n";

# Binary path
my $binary = "./target/release/hegel-pm";

# Check if binary exists
unless (-x $binary) {
    die "Error: Binary not found at $binary\n" .
        "Please run: cargo build --release --features server\n";
}

# Check if jq is available
unless (system("which jq > /dev/null 2>&1") == 0) {
    die "Error: jq is not installed. Install it to validate JSON output.\n";
}

# Define test cases: [description, command_args, requires_project_arg]
my @test_cases = (
    {
        desc => "discover list",
        args => ["discover", "list", "--json"],
        requires_project => 0,
    },
    {
        desc => "discover all",
        args => ["discover", "all", "--json"],
        requires_project => 0,
    },
    {
        desc => "discover all with sort",
        args => ["discover", "all", "--json", "--sort-by", "name"],
        requires_project => 0,
    },
    {
        desc => "discover show",
        args => ["discover", "show", "PROJECT_NAME", "--json"],
        requires_project => 1,
    },
);

print "=== JSON Smoke Test ===\n\n";

if ($dry_run) {
    print "DRY RUN MODE - showing what would be tested:\n\n";
}

my $passed = 0;
my $failed = 0;
my $skipped = 0;

# First, get a project name if we need it for any tests
my $project_name = undef;
if (grep { $_->{requires_project} } @test_cases) {
    print "Getting project list for tests that require project name...\n";
    my $list_output = `$binary discover list --json 2>/dev/null`;
    if ($? == 0 && $list_output) {
        # Parse JSON to get first project name
        my $decoded = `echo '$list_output' | jq -r '.projects[0].name' 2>/dev/null`;
        chomp($decoded);
        if ($decoded && $decoded ne "null" && $decoded ne "") {
            $project_name = $decoded;
            print "Using project '$project_name' for project-specific tests\n\n";
        }
    }
}

for my $test (@test_cases) {
    my $desc = $test->{desc};
    my @args = @{$test->{args}};

    # Skip tests that require a project if we don't have one
    if ($test->{requires_project}) {
        unless (defined $project_name) {
            print "⊘ SKIP: $desc (no projects found)\n";
            $skipped++;
            next;
        }
        # Replace PROJECT_NAME placeholder
        @args = map { $_ eq "PROJECT_NAME" ? $project_name : $_ } @args;
    }

    my $cmd = join(" ", $binary, @args);

    if ($dry_run) {
        print "Would test: $cmd\n";
        next;
    }

    if ($verbose) {
        print "Testing: $cmd\n";
    } else {
        print "Testing $desc... ";
    }

    # Run command and capture output
    my $output = `$cmd 2>&1`;
    my $exit_code = $? >> 8;

    if ($exit_code != 0) {
        print "✗ FAIL (command failed with exit code $exit_code)\n";
        if ($verbose || length($output) < 500) {
            print "  Output: $output\n";
        }
        $failed++;
        next;
    }

    # Validate JSON with jq
    my $jq_result = system("echo '$output' | jq empty 2>/dev/null");

    if ($jq_result == 0) {
        print "✓ PASS\n";
        if ($verbose) {
            print "  Output preview: " . substr($output, 0, 100) . "...\n";
        }
        $passed++;
    } else {
        print "✗ FAIL (invalid JSON)\n";
        if ($verbose || length($output) < 500) {
            print "  Output: $output\n";
        }
        # Show jq error
        my $jq_error = `echo '$output' | jq empty 2>&1`;
        print "  jq error: $jq_error\n";
        $failed++;
    }
}

# Summary
print "\n=== Summary ===\n";
print "Passed:  $passed\n";
print "Failed:  $failed\n";
print "Skipped: $skipped\n";
print "Total:   " . scalar(@test_cases) . "\n";

if ($dry_run) {
    print "\nRun without --dry-run to execute tests\n";
    exit 0;
}

exit($failed > 0 ? 1 : 0);
