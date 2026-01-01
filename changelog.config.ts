export default {
  // Repository configuration
  repo: "Ripwords/rk-configurator",

  // Output file
  output: "CHANGELOG.md",

  // Include contributors section
  contributors: true,

  // GitHub release integration
  github: {
    repo: "Ripwords/rk-configurator",
    token:
      process.env.CHANGELOGEN_TOKENS_GITHUB ||
      process.env.GITHUB_TOKEN ||
      process.env.GH_TOKEN,
  },

  // Tag prefix for releases (matches tauri-action tagName format)
  tags: {
    prefix: "v",
  },

  // Changelog sections
  types: {
    feat: { title: "🚀 Features" },
    fix: { title: "🐛 Bug Fixes" },
    perf: { title: "⚡ Performance Improvements" },
    refactor: { title: "♻️ Code Refactoring" },
    docs: { title: "📝 Documentation" },
    style: { title: "💄 Styles" },
    test: { title: "✅ Tests" },
    build: { title: "👷 Build System" },
    ci: { title: "🔧 CI/CD" },
    chore: { title: "🔨 Chores" },
    revert: { title: "⏪ Reverts" },
  },
};
