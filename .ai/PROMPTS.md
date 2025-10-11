# Common AI Prompts and Tasks

This document contains common prompts and tasks for AI assistants working on the ENTSO-E library project.

## Quick Context Loading

When starting a new session:

```
Read all files in the .ai/ directory to understand the project context.
```

## Common Development Tasks

### 1. Implement a New Feature

```
Implement [feature name] following these requirements:
- [Requirement 1]
- [Requirement 2]
- Follow the coding standards in .ai/DEVELOPMENT_GUIDE.md
- Add tests and documentation
- Use the architecture outlined in .ai/PROJECT_OVERVIEW.md
```

### 2. Fix a Bug

```
There's a bug where [description]. 
- Check the relevant code in [file/module]
- Add a failing test that reproduces the bug
- Fix the bug
- Ensure all tests pass
- Explain what caused the bug
```

### 3. Add Tests

```
Add comprehensive tests for [module/function]:
- Unit tests for edge cases
- Integration tests for the full workflow
- Use fixtures from tests/fixtures/
- Follow the testing strategy in .ai/DEVELOPMENT_GUIDE.md
```

### 4. Parse a New XML Response Type

```
Add support for parsing [data type] from ENTSO-E API:
1. Create a fixture file with a sample XML response
2. Define the model struct
3. Implement the parser
4. Add tests with the fixture
5. Add a client method to fetch this data
6. Create an example
7. Update documentation
```

### 5. Debug API Issues

```
The API call is failing with [error]. Help me debug:
1. Check the request parameters against .ai/API_REFERENCE.md
2. Verify the EIC code is correct
3. Check timestamp formatting
4. Add logging to see the raw request and response
5. Compare with working examples
```

### 6. Add Documentation

```
Add comprehensive documentation for [module/function]:
- Follow the documentation style in DEVELOPMENT_GUIDE.md
- Include a summary, detailed description, and examples
- Document all parameters and return values
- Document error conditions
- Add a working example in the doc comment
- Use minimal emojis
```

### 7. Optimize Performance

```
Profile and optimize [function/module]:
1. Add benchmarks to measure current performance
2. Identify bottlenecks
3. Implement optimizations following the coding standards
4. Verify performance improvement with benchmarks
5. Ensure tests still pass
```

### 8. Refactor Code

```
Refactor [module] to improve [code quality/maintainability/performance]:
- Keep the same public API
- Ensure all tests still pass
- Follow Rust idioms and best practices
- Update documentation if needed
```

## Code Review Prompts

### Self-Review

```
Review the code I just wrote:
1. Check against the coding standards in DEVELOPMENT_GUIDE.md
2. Verify error handling is proper
3. Ensure documentation is complete
4. Check for potential edge cases
5. Suggest improvements
6. Check for excessive emoji usage
```

### Security Review

```
Review the security of [module]:
- Check for proper API token handling
- Ensure no secrets in logs
- Verify input validation
- Check for injection vulnerabilities
```

## Debugging Prompts

### Parse Error

```
I'm getting a parse error when handling [XML response]:
1. Show me the relevant XML structure
2. Check the parser logic against the XML
3. Compare with the schema in .ai/API_REFERENCE.md
4. Identify what's different or missing
5. Fix the parser
```

### Type Error

```
There's a type error in [location]:
[error message]

Help me understand and fix this error.
```

### Test Failure

```
This test is failing:
[test name and error]

Help me debug:
1. Understand what the test is checking
2. Identify why it's failing
3. Determine if it's a test issue or code issue
4. Fix the problem
```

## Research Prompts

### Understanding ENTSO-E

```
Explain [concept] in the context of ENTSO-E:
- What is it?
- How does it relate to our library?
- Are there any special considerations?
- Reference API_REFERENCE.md
```

### API Clarification

```
I need to understand [API parameter/response field]:
- Check API_REFERENCE.md
- Explain what it means
- Show example values
- Explain how to use it in our library
```

### Rust Pattern

```
What's the best Rust pattern for [problem]?
- Show examples
- Explain trade-offs
- Recommend the best approach for our use case
- Follow the principles in DEVELOPMENT_GUIDE.md
```

## Documentation Prompts

### Generate Examples

```
Create a complete example showing how to [use case]:
- Use realistic parameters
- Handle errors properly
- Add comments explaining each step
- Make it runnable
```

### Update README

```
Update the README.md with:
- Installation instructions
- Quick start guide
- Basic usage examples
- Link to full documentation
```

### API Documentation

```
Generate API documentation for [module]:
- Document all public functions, types, and traits
- Include examples for each
- Follow the style in DEVELOPMENT_GUIDE.md
- Ensure doc tests compile and run
- Use minimal emojis
```

## Testing Prompts

### Generate Test Cases

```
Generate comprehensive test cases for [function]:
- Happy path tests
- Error cases
- Edge cases (empty input, max values, etc.)
- Invalid input tests
- Use fixtures where appropriate
```

### Create Test Fixture

```
Create a test fixture for [scenario]:
- Use a realistic XML response
- Sanitize any sensitive data
- Save to tests/fixtures/[name].xml
- Add a test that uses this fixture
```

### Integration Test

```
Create an integration test for the full workflow:
1. Initialize client
2. Make API request
3. Parse response
4. Verify results
5. Handle errors
Make it work with and without an actual API token.
```

## Maintenance Prompts

### Update Dependencies

```
Update dependencies:
1. Run cargo update
2. Check for breaking changes
3. Update code if needed
4. Run all tests
5. Update Cargo.toml if major versions changed
```

### Fix Clippy Warnings

```
Fix all clippy warnings:
- Run cargo clippy
- Address each warning appropriately
- Don't disable warnings without good reason
- Explain any suppressed warnings
```

### Improve Error Messages

```
Improve error messages in [module]:
- Make them more descriptive
- Add context (what operation failed, why)
- Suggest solutions when possible
- Follow error handling guidelines
```

## AI Assistant Tips

When working on this project:

1. **Check the .ai/ directory first** - It contains context and guidelines
2. **Follow the architecture** - Don't introduce patterns that conflict with the design
3. **Test thoroughly** - Add tests for new code
4. **Document as you go** - Don't leave documentation for later
5. **Use fixtures** - Don't make real API calls in tests unnecessarily
6. **Handle errors properly** - No unwrap/panic in library code
7. **Minimal emojis** - Use emojis sparingly or not at all
8. **Ask clarifying questions** - If requirements are unclear, ask before implementing

## Example Session Flow

Here's a typical flow for implementing a new feature:

```
Session Start:
> Read all files in .ai/ to understand the project

Planning:
> I want to add support for fetching load data. 
> Review API_REFERENCE.md and explain what parameters I need.

Implementation:
> Implement load data fetching following the same pattern as price fetching.
> Include model, parser, client method, tests, and example.

Testing:
> Create test fixtures for load data responses
> Add unit and integration tests

Documentation:
> Document the new functionality
> Add an example to README.md

Review:
> Review the code against DEVELOPMENT_GUIDE.md
> Run cargo test, cargo clippy, cargo fmt
> Ensure everything passes
```
