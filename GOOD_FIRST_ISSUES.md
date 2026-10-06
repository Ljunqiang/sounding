# Good First Issues - Stellar Pathfinder

52 simple, beginner-friendly issues to help new contributors get started!

Each issue takes approximately **3-5 minutes** and helps you learn the codebase while making meaningful contributions.

---

## 📚 Documentation Issues (12)

### 1. Add JSDoc comment to `formatBalance` function
**File**: `server/index.html` (line ~2234)
**Task**: Add JSDoc explaining parameters and return value
```javascript
/**
 * Format balance for display with appropriate decimal precision
 * @param {number} amount - The balance amount to format
 * @returns {string} Formatted balance string
 */
```

### 2. Add code example to README for API usage
**File**: `README.md`
**Task**: Add a curl example showing how to query a corridor
```bash
curl "https://stellar-pathfinder.onrender.com/api/corridor?to=NGNC&live=1"
```

### 3. Document the verdict thresholds in glossary
**File**: `docs/glossary.md`
**Task**: Add exact percentage thresholds for GOOD/FAIR/POOR/UNUSABLE

### 4. Add inline comment explaining hash-chain verification
**File**: `runstore/*.go`
**Task**: Add 1-2 line comment explaining why SHA256 hash-chaining is used

### 5. Create quickstart guide
**File**: `docs/QUICKSTART.md` (new file)
**Task**: 5-step guide: clone → install → run → open browser → view results

### 6. Add tooltip text to "Verified on Stellar" badge
**File**: `server/index.html`
**Task**: Improve the title attribute text to explain what verification means

### 7. Document environment variables
**File**: `README.md`
**Task**: Add section listing `WAYFARE_DATA_DIR`, `WAYFARE_LOG_LEVEL`, `PORT`

### 8. Add comment explaining decimal string usage
**File**: `route/money.go`
**Task**: Explain why money is represented as strings not floats

### 9. Create troubleshooting entry for "No corridors available"
**File**: `docs/troubleshooting.md`
**Task**: Add solution for when corridor selector shows empty

### 10. Add Go package documentation
**File**: `attestation/attestation.go`
**Task**: Add package-level comment explaining the attestation package purpose

### 11. Document Soroban contract functions
**File**: `contract/corridor-attestation/README.md`
**Task**: Add usage examples for `get_count()` and `has_attestations()`

### 12. Add CHANGELOG entry template
**File**: `CHANGELOG.md` (new file)
**Task**: Create changelog with template for future releases

---

## 🎨 UI/UX Issues (10)

### 13. Add loading spinner aria-label
**File**: `server/index.html`
**Task**: Add `aria-label="Loading corridor data"` to loading dots

### 14. Improve button text: "Measure live" → "Measure Now"
**File**: `server/index.html`
**Task**: Update button text for clarity

### 15. Add placeholder text to sizes input
**File**: `server/index.html`
**Task**: Add `placeholder="e.g., 10,100,1000"` to sizes input

### 16. Add focus visible styling to wallet buttons
**File**: `server/index.html`
**Task**: Ensure connect/disconnect wallet buttons have `:focus-visible` styles

### 17. Add success message after wallet connection
**File**: `server/index.html`
**Task**: Display "✓ Wallet connected successfully" toast/message

### 18. Make corridor selector label more descriptive
**File**: `server/index.html`
**Task**: Change "Corridor" label to "Select Corridor"

### 19. Add visual separator between controls
**File**: `server/index.html`
**Task**: Add subtle border-right between control groups

### 20. Improve empty state icon
**File**: `server/index.html`
**Task**: Replace text glyph with SVG icon for better visuals

### 21. Add "Copy address" button for connected wallet
**File**: `server/index.html`
**Task**: Add copy-to-clipboard button next to wallet address

### 22. Add keyboard shortcut hint (Ctrl+M for Measure)
**File**: `server/index.html`
**Task**: Display keyboard shortcut in button tooltip

---

## 🧪 Testing Issues (8)

### 23. Add test case for empty corridor ID
**File**: `server/attestations_test.go` (new file)
**Task**: Test that `/api/attestations/` returns 400

### 24. Add test for formatBalance with zero
**File**: `server/index.html` (inline test)
**Task**: Add console assertion `formatBalance(0) === "0.0000000"`

### 25. Add test for negative balance handling
**File**: Tests for balance display
**Task**: Verify negative balances don't display

### 26. Test wallet connection with no extension
**File**: Test Freighter detection
**Task**: Verify error message when Freighter not installed

### 27. Add test for corridor ID formatting
**File**: `attestation/attestation_test.go` (new file)
**Task**: Test `BuildCorridorID("USDC", "NGNC")` returns `"USDC→NGNC"`

### 28. Test Soroban badge with no attestations
**File**: Test badge display logic
**Task**: Verify badge doesn't show when no attestations exist

### 29. Add test for truncated address display
**File**: Test `formatAddress()` function
**Task**: Verify "GABCD...WXYZ" formatting works correctly

### 30. Test deep link URL generation
**File**: Test URL generation functions
**Task**: Verify StellarX and StellarTerm URLs are correct

---

## 🔧 Code Quality Issues (10)

### 31. Extract magic number 100 to constant MAX_ATTESTATIONS
**File**: `contract/corridor-attestation/src/lib.rs`
**Task**: Replace hardcoded 100 with named constant

### 32. Replace hardcoded color with CSS variable
**File**: `server/index.html`
**Task**: Replace `#FFFFFF` in button with `var(--panel)`

### 33. Add default case to verdict switch
**File**: JavaScript verdict handling
**Task**: Add `default: return ''` to verdict mark function

### 34. Extract repeated "USDC→NGNC" to test constant
**File**: Test files
**Task**: Define `const TEST_CORRIDOR = "USDC→NGNC"` at top of test files

### 35. Replace inline styles with CSS classes
**File**: `server/index.html` (personalized panel)
**Task**: Move inline `style=""` to CSS class `.personalized-panel`

### 36. Add input validation to sizes field
**File**: `server/index.html`
**Task**: Add `pattern` attribute for comma-separated numbers

### 37. Extract wallet provider names to constants
**File**: JavaScript wallet code
**Task**: Define `const FREIGHTER = 'freighter'` etc.

### 38. Add early return for null checks
**File**: `attestation/attestation.go`
**Task**: Add early return if `attestation == nil`

### 39. Use string interpolation instead of concatenation
**File**: Various JavaScript files
**Task**: Replace `"text" + var + "more"` with template literals

### 40. Add missing error type check
**File**: Error handling code
**Task**: Add type assertion for error objects

---

## ♿ Accessibility Issues (6)

### 41. Add ARIA label to corridor selector
**File**: `server/index.html`
**Task**: Add `aria-describedby` linking to help text

### 42. Add role="status" to measurement results
**File**: Results display
**Task**: Ensure screen readers announce new results

### 43. Improve color contrast for muted text
**File**: CSS variables
**Task**: Check muted text meets WCAG AA (4.5:1 ratio)

### 44. Add alt text to wallet badge icons
**File**: Wallet badge HTML
**Task**: Add descriptive alt text or aria-label

### 45. Add keyboard navigation to DEX links
**File**: StellarX/StellarTerm links
**Task**: Ensure links are keyboard accessible (already done, verify)

### 46. Add focus trap to wallet connection modal
**File**: Wallet selection dialog
**Task**: Prevent focus escaping during wallet selection

---

## 🌐 Internationalization Prep (4)

### 47. Extract hardcoded "Loading..." text
**File**: `server/index.html`
**Task**: Move to const `MESSAGES = { loading: "Loading..." }`

### 48. Extract error messages to constants
**File**: Error handling
**Task**: Define `ERROR_MESSAGES` object at top of file

### 49. Extract button labels to constants
**File**: Button text
**Task**: Define `BUTTON_LABELS = { measure: "Measure Now", ... }`

### 50. Standardize date format strings
**File**: Date formatting code
**Task**: Use ISO 8601 format consistently

---

## 🚀 Performance & Polish (2)

### 51. Add debounce to sizes input validation
**File**: Sizes input handler
**Task**: Only validate after 300ms pause in typing

### 52. Cache wallet balance requests
**File**: Balance fetching code
**Task**: Add 30-second cache to prevent excessive Horizon queries

---

## 📋 How to Claim an Issue

1. **Comment on the issue**: "I'd like to work on this!"
2. **Wait for assignment**: Maintainer will assign you
3. **Create a branch**: `git checkout -b fix/issue-number-description`
4. **Make your changes**: Follow the acceptance criteria
5. **Test locally**: Run `go test` or test in browser
6. **Submit PR**: Reference the issue number
7. **Celebrate**: You're a contributor! 🎉

---

## 🏷️ Labels

All issues will be tagged with:
- `good first issue` - Perfect for newcomers
- `help wanted` - Actively seeking contributors
- `documentation` / `ui` / `testing` / `accessibility` - Category labels
- `3-min-task` - Super quick wins

---

## 💡 Tips for Success

1. **Ask questions!** No question is too basic
2. **One issue at a time** - Master one before taking another
3. **Read CONTRIBUTING.md** - Understand our workflow
4. **Test your changes** - Even small changes need verification
5. **Follow existing patterns** - Match the style of surrounding code
6. **Have fun!** - You're making Stellar better ⭐

---

**Ready to contribute?** Pick an issue and let's build together! 🚀
