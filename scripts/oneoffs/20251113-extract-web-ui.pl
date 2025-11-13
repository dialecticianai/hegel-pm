#!/usr/bin/env perl
use strict;
use warnings;
use File::Path qw(make_path remove_tree);
use File::Copy qw(copy move);
use File::Spec;
use File::Basename;
use Cwd qw(abs_path getcwd);

# Migration script: Extract web UI from hegel-pm to hegel-pm-web
# Usage: ./scripts/oneoffs/20251113-extract-web-ui.pl [--dry-run]

my $dry_run = grep { $_ eq '--dry-run' } @ARGV;
my $script_dir = dirname(abs_path($0));
my $repo_root = abs_path("$script_dir/../..");
my $target_root = abs_path("$repo_root/../hegel-pm-web");

print "=== Web UI Extraction Script ===\n";
print "Mode: " . ($dry_run ? "DRY RUN" : "EXECUTING") . "\n";
print "Source: $repo_root\n";
print "Target: $target_root\n\n";

# Track operations for summary
my @operations;
my $errors = 0;

sub log_op {
    my ($msg) = @_;
    push @operations, $msg;
    print "  $msg\n";
}

sub execute {
    my ($msg, $code) = @_;
    log_op($msg);
    return if $dry_run;
    eval { $code->(); };
    if ($@) {
        print "    ERROR: $@\n";
        $errors++;
    }
}

# Step 1: Create target repository structure
print "Step 1: Creating target repository structure...\n";
for my $dir ('src', 'scripts', 'tests', '.ddd/feat/extract-web-ui') {
    execute("Create directory: $target_root/$dir", sub {
        make_path("$target_root/$dir") or die "Failed to create $dir: $!";
    });
}

# Step 2: Move source directories
print "\nStep 2: Moving source directories...\n";
my @src_dirs = qw(http data_layer client);
for my $dir (@src_dirs) {
    my $src = "$repo_root/src/$dir";
    my $dst = "$target_root/src/$dir";
    if (-d $src) {
        execute("Move directory: src/$dir", sub {
            system("cp", "-R", $src, $dst) == 0 or die "Failed to copy $dir: $!";
            remove_tree($src) or die "Failed to remove $src: $!";
        });
    } else {
        log_op("SKIP (not found): src/$dir");
    }
}

# Step 3: Move source files
print "\nStep 3: Moving source files...\n";
my @src_files = qw(server_mode.rs benchmark_mode.rs api_types.rs);
for my $file (@src_files) {
    my $src = "$repo_root/src/$file";
    my $dst = "$target_root/src/$file";
    if (-f $src) {
        execute("Move file: src/$file", sub {
            copy($src, $dst) or die "Failed to copy $file: $!";
            unlink($src) or die "Failed to remove $src: $!";
        });
    } else {
        log_op("SKIP (not found): src/$file");
    }
}

# Step 4: Move top-level directories
print "\nStep 4: Moving top-level directories...\n";
my @top_dirs = qw(frontends static tests);
for my $dir (@top_dirs) {
    my $src = "$repo_root/$dir";
    my $dst = "$target_root/$dir";
    if (-d $src) {
        execute("Move directory: $dir/", sub {
            system("cp", "-R", $src, $dst) == 0 or die "Failed to copy $dir: $!";
            remove_tree($src) or die "Failed to remove $src: $!";
        });
    } else {
        log_op("SKIP (not found): $dir/");
    }
}

# Step 5: Move top-level files
print "\nStep 5: Moving top-level files...\n";
my @top_files = ('index.html', 'scripts/restart-server.sh');
for my $file (@top_files) {
    my $src = "$repo_root/$file";
    my $dst = "$target_root/$file";
    if (-f $src) {
        execute("Move file: $file", sub {
            my $dst_dir = dirname($dst);
            make_path($dst_dir) unless -d $dst_dir;
            copy($src, $dst) or die "Failed to copy $file: $!";
            unlink($src) or die "Failed to remove $src: $!";
        });
    } else {
        log_op("SKIP (not found): $file");
    }
}

# Step 6: Copy scaffold files
print "\nStep 6: Copying scaffold files...\n";
my @scaffold_files = ('LICENSE');
for my $file (@scaffold_files) {
    my $src = "$repo_root/$file";
    my $dst = "$target_root/$file";
    if (-f $src) {
        execute("Copy file: $file", sub {
            copy($src, $dst) or die "Failed to copy $file: $!";
        });
    } else {
        log_op("SKIP (not found): $file");
    }
}

# Step 7: Create .gitignore for hegel-pm-web
print "\nStep 7: Creating .gitignore...\n";
my $gitignore_content = <<'EOF';
/target/
/static/*.js
/static/*.wasm
/static/index.html
**/*.rs.bk
Cargo.lock
.DS_Store
EOF

execute("Create file: .gitignore", sub {
    open my $fh, '>', "$target_root/.gitignore" or die "Failed to create .gitignore: $!";
    print $fh $gitignore_content;
    close $fh;
});

# Step 8: Create skeleton Cargo.toml files
print "\nStep 8: Creating skeleton Cargo.toml files...\n";

my $pm_web_cargo = <<'EOF';
# TODO: Complete this Cargo.toml manually
# See MANUAL_EDITS.md for instructions
#
# Required structure:
# [package]
# name = "hegel-pm-web"
# version = "0.0.1"
# edition = "2021"
#
# [dependencies]
# hegel = { path = "../hegel-cli" }
# hegel-pm = { path = "../hegel-pm" }
# ... (copy web dependencies from original hegel-pm)
#
# [features]
# default = ["warp-backend"]
# server = []
# warp-backend = ["warp"]
# axum-backend = ["axum", "tower", "tower-http"]
#
# [[bin]]
# name = "hegel-pm-web"
# path = "src/main.rs"
# required-features = ["server"]
#
# [lib]
# crate-type = ["cdylib", "rlib"]
EOF

execute("Create file: Cargo.toml (skeleton)", sub {
    open my $fh, '>', "$target_root/Cargo.toml" or die "Failed to create Cargo.toml: $!";
    print $fh $pm_web_cargo;
    close $fh;
});

# Step 9: Create skeleton lib.rs and main.rs
print "\nStep 9: Creating skeleton source files...\n";

my $pm_web_lib = <<'EOF';
// TODO: Declare web modules manually
// See MANUAL_EDITS.md for instructions
//
// Required modules:
// pub mod http;
// pub mod data_layer;
// pub mod client;
// pub mod server_mode;
// pub mod benchmark_mode;
// pub mod api_types;
EOF

execute("Create file: src/lib.rs (skeleton)", sub {
    open my $fh, '>', "$target_root/src/lib.rs" or die "Failed to create lib.rs: $!";
    print $fh $pm_web_lib;
    close $fh;
});

my $pm_web_main = <<'EOF';
// TODO: Copy server/benchmark logic from original hegel-pm main.rs
// See MANUAL_EDITS.md for instructions
//
// Remove CLI discover commands (those stay in hegel-pm)
// Keep server mode and benchmark mode entry points
EOF

execute("Create file: src/main.rs (skeleton)", sub {
    open my $fh, '>', "$target_root/src/main.rs" or die "Failed to create main.rs: $!";
    print $fh $pm_web_main;
    close $fh;
});

# Step 10: Create README stub
print "\nStep 10: Creating README stub...\n";

my $pm_web_readme = <<'EOF';
# hegel-pm-web

Web dashboard for Hegel projects with HTTP server and multiple frontend implementations.

TODO: Complete this README after migration is verified working.

## Development

See MANUAL_EDITS.md for setup instructions.
EOF

execute("Create file: README.md (stub)", sub {
    open my $fh, '>', "$target_root/README.md" or die "Failed to create README.md: $!";
    print $fh $pm_web_readme;
    close $fh;
});

# Step 11: Generate MANUAL_EDITS.md checklists
print "\nStep 11: Generating MANUAL_EDITS.md checklists...\n";

my $pm_manual_edits = <<'EOF';
# Manual Edits Required for hegel-pm

After running the migration script, complete these manual edits:

## 1. Edit Cargo.toml

Remove these dependencies:
- [ ] warp, axum, tower, tower-http, async-trait
- [ ] sycamore, wasm-bindgen, wasm-bindgen-futures, gloo-net, web-sys, console_error_panic_hook
- [ ] dashmap, reqwest
- [ ] tokio (evaluate - keep only if CLI needs async)

Remove sections:
- [ ] Remove [features] section entirely
- [ ] Remove [[bin]] required-features constraint
- [ ] Change [lib] crate-type from ["cdylib", "rlib"] to ["rlib"]

## 2. Edit src/lib.rs

Remove module declarations:
- [ ] Remove: mod http;
- [ ] Remove: mod data_layer;
- [ ] Remove: mod client;
- [ ] Remove: mod server_mode;
- [ ] Remove: mod benchmark_mode;
- [ ] Remove: mod api_types;

Keep:
- [ ] Verify: pub mod discovery;
- [ ] Verify: mod cli; (if present)
- [ ] Verify: mod debug; (if present)
- [ ] Verify: mod test_helpers; (if present)

## 3. Edit src/main.rs

Remove server/benchmark logic:
- [ ] Remove server mode launch code
- [ ] Remove benchmark mode code
- [ ] Keep only CLI command dispatch (discover, hegel)

## 4. Edit scripts/test.sh

Remove frontend build:
- [ ] Remove trunk build commands
- [ ] Remove frontend-specific logic
- [ ] Keep cargo build and cargo test for CLI/lib

## 5. Verify

- [ ] Run: cargo build
- [ ] Run: cargo test
- [ ] Verify: hegel-pm discover list works
- [ ] Check test count: ~45-50 tests pass
EOF

execute("Create file: MANUAL_EDITS.md (hegel-pm)", sub {
    open my $fh, '>', "$repo_root/MANUAL_EDITS.md" or die "Failed to create MANUAL_EDITS.md: $!";
    print $fh $pm_manual_edits;
    close $fh;
});

my $pm_web_manual_edits = <<'EOF';
# Manual Edits Required for hegel-pm-web

After running the migration script, complete these manual edits:

## 1. Complete Cargo.toml

The skeleton file needs completion. Copy from original hegel-pm/Cargo.toml:

- [ ] Add [package] metadata (name, version, edition, authors, description, license, repository)
- [ ] Add dependency: hegel = { path = "../hegel-cli" }
- [ ] Add dependency: hegel-pm = { path = "../hegel-pm" }
- [ ] Copy all web dependencies from original Cargo.toml:
  - warp/axum backend deps
  - sycamore and WASM deps
  - tokio, dashmap, reqwest
  - All target-specific deps
- [ ] Add [dev-dependencies] if needed
- [ ] Add [features] section
- [ ] Add [[bin]] section
- [ ] Add [lib] section with crate-type = ["cdylib", "rlib"]

## 2. Complete src/lib.rs

Replace TODO with actual module declarations:

- [ ] Add: pub mod http;
- [ ] Add: pub mod data_layer;
- [ ] Add: pub mod client;
- [ ] Add: pub mod server_mode;
- [ ] Add: pub mod benchmark_mode;
- [ ] Add: pub mod api_types;

## 3. Complete src/main.rs

Copy server/benchmark logic from original hegel-pm/src/main.rs:

- [ ] Copy server mode launch code
- [ ] Copy benchmark mode code
- [ ] Remove CLI discover commands
- [ ] Entry point: server or benchmark based on args

## 4. Fix imports in moved files

Update discovery imports from local to hegel-pm library:

- [ ] In src/data_layer/worker.rs: Change crate::discovery to hegel_pm::discovery
- [ ] In src/data_layer/mod.rs: Check for discovery imports
- [ ] In src/http/warp_backend.rs: Check for discovery imports
- [ ] In src/http/axum_backend.rs: Check for discovery imports
- [ ] Search all moved files for: use crate::discovery
- [ ] Replace with: use hegel_pm::discovery

Tip: Use find/replace or: grep -r "crate::discovery" src/

## 5. Handle debug.rs and test_helpers.rs

These files stayed in hegel-pm. If web code uses them:

- [ ] Check if data_layer, http, etc. import debug or test_helpers
- [ ] If yes, either: (a) duplicate them here, or (b) add to hegel-pm lib exports

## 6. Update scripts/test.sh

The moved script needs updates:

- [ ] Keep trunk build for frontend
- [ ] Keep cargo build --features server for backend
- [ ] Keep cargo test --features server
- [ ] Remove references to hegel-pm binary
- [ ] Update paths if needed

## 7. Verify scripts/restart-server.sh

- [ ] Check paths are correct
- [ ] Verify it references hegel-pm-web binary
- [ ] Test script execution

## 8. Verify

- [ ] Run: cargo build --features server
- [ ] Run: cargo test --features server
- [ ] Check test count: ~93 tests pass (lib tests)
- [ ] Verify integration tests run (5 test files in tests/)
- [ ] Run: ./scripts/test.sh (full cycle)
- [ ] Run: cargo run --bin hegel-pm-web --features server
- [ ] Verify: Server starts on localhost:3030
EOF

execute("Create file: MANUAL_EDITS.md (hegel-pm-web)", sub {
    open my $fh, '>', "$target_root/MANUAL_EDITS.md" or die "Failed to create MANUAL_EDITS.md: $!";
    print $fh $pm_web_manual_edits;
    close $fh;
});

# Summary
print "\n" . ("=" x 60) . "\n";
print "MIGRATION SCRIPT SUMMARY\n";
print "=" x 60 . "\n";
print "Total operations: " . scalar(@operations) . "\n";
print "Errors: $errors\n";
print "\n";

if ($dry_run) {
    print "DRY RUN COMPLETE - No files were actually moved.\n";
    print "Review operations above and run without --dry-run to execute.\n";
} else {
    if ($errors == 0) {
        print "MIGRATION COMPLETE!\n\n";
        print "Next steps:\n";
        print "1. Review MANUAL_EDITS.md in both repositories\n";
        print "2. Complete manual edits in hegel-pm first\n";
        print "3. Complete manual edits in hegel-pm-web\n";
        print "4. Run validation: cargo build && cargo test in both repos\n";
    } else {
        print "MIGRATION COMPLETED WITH ERRORS!\n";
        print "Review errors above and fix manually.\n";
    }
}

exit($errors > 0 ? 1 : 0);
