use std::os::unix::fs::PermissionsExt;
use std::{fs, path::Path};

const HOOK: &str = r#"
#!/bin/sh
STAGED_FILES=$(git diff --cached --name-only --diff-filter=ACMR | sed 's| |\\ |g')

echo "---------------------------"
echo "- ✨  Running clippy   ✨ -"
echo "---------------------------"
cargo clippy
LINTER_EXIT_CODE=$?

if [ $LINTER_EXIT_CODE -ne 0 ]; then
    echo "---------------------------"
    echo "- ❌ Fix clippy errors ❌ -"
    echo "---------------------------"

    exit 1
else
    echo "✅"
fi

echo "---------------------------"
echo "- ✨ Running formatter ✨ -"
echo "---------------------------"
cargo fmt
echo "✅"

git add -f $STAGED_FILES

echo "---------------------------"
echo "- ✨   Running tests   ✨ -"
echo "---------------------------"
cargo test
TEST_EXIT_CODE=$?

if [ $TEST_EXIT_CODE -ne 0 ]; then
    echo "---------------------------"
    echo "- ❌  Fix test errors  ❌ -"
    echo "---------------------------"
    exit 1
else
    echo "✅"
fi

echo "--------------------------------------"
echo "- 🎉 linted, formatted and tested 🎉 -"
echo "--------------------------------------"
exit 0

"#;

fn main() {
    let path = Path::new(".git").join("hooks").join("pre-commit");
    fs::write(&path, HOOK).expect("check git is initialized and you have a folder in .git/hooks");
    let mut perms = fs::metadata(&path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms).ok();
}
