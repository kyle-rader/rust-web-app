# Testing Guide for Automata Games Client

This document provides a comprehensive guide to testing the SvelteJS frontend application.

## Testing Architecture

The testing setup uses a multi-layered approach:

- **Unit Tests**: Test individual components and functions in isolation using Vitest and `@testing-library/svelte`
- **Integration Tests**: Test user flows and page interactions using Playwright with API mocking
- **Test Utilities**: Shared helper functions and mock data for consistent testing

## Quick Start

### Running Tests

```bash
# Run all tests (unit + integration)
npm test

# Run only unit tests
npm run test:unit

# Run unit tests in watch mode (for development)
npm run test:unit:watch

# Run unit tests with coverage
npm run test:unit:coverage

# Run only integration tests
npm run test:integration

# Run integration tests with UI
npm run test:integration:ui
```

## Unit Testing

### Test Structure

Unit tests are located alongside the components they test:
- `ComponentName.test.ts` - Tests for `ComponentName.svelte`
- `functionName.test.ts` - Tests for utility functions
- `storeName.test.ts` - Tests for Svelte stores

**Note**: Avoid using `+` prefix in test filenames (e.g., `+page.test.ts`) as these are reserved by SvelteKit. Use `page.test.ts` instead.

### Writing Unit Tests

#### Basic Component Test

```typescript
import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import MyComponent from './MyComponent.svelte';

describe('MyComponent', () => {
  it('renders correctly', () => {
    render(MyComponent);
    expect(screen.getByText('Hello World')).toBeInTheDocument();
  });
});
```

#### Testing User Interactions

```typescript
import { fireEvent } from '@testing-library/svelte';

it('handles button click', async () => {
  const { component } = render(MyComponent);
  const mockHandler = vi.fn();
  component.$on('click', mockHandler);
  
  const button = screen.getByRole('button');
  await fireEvent.click(button);
  
  expect(mockHandler).toHaveBeenCalled();
});
```

#### Testing Form Components

```typescript
it('submits form data', async () => {
  const { component } = render(LoginForm);
  const mockSubmit = vi.fn();
  component.$on('login', mockSubmit);
  
  const emailInput = screen.getByLabelText(/email/i);
  const passwordInput = screen.getByLabelText(/password/i);
  
  await fireEvent.input(emailInput, { target: { value: 'test@example.com' } });
  await fireEvent.input(passwordInput, { target: { value: 'password123' } });
  
  const submitButton = screen.getByRole('button', { name: /login/i });
  await fireEvent.click(submitButton);
  
  expect(mockSubmit).toHaveBeenCalledWith(
    expect.objectContaining({
      detail: {
        email: 'test@example.com',
        password: 'password123'
      }
    })
  );
});
```

### Testing Utilities

Use the shared test utilities in `src/test/utils.ts`:

```typescript
import { createMockUser, renderComponent, fillForm } from '../../test/utils';

// Create mock data
const mockUser = createMockUser({ display_name: 'Test User' });

// Render component with props
const { container } = renderComponent(MyComponent, { user: mockUser });

// Fill form fields
await fillForm({
  email: 'test@example.com',
  password: 'password123'
});
```

### Mocking Dependencies

#### Mocking SvelteKit Modules

```typescript
import { vi } from 'vitest';

vi.mock('$app/navigation', () => ({
  goto: vi.fn(),
  preloadData: vi.fn()
}));
```

#### Mocking API Calls

```typescript
import { mockFetch } from '../../test/utils';

const mockResponse = { id: 1, name: 'Test' };
global.fetch = mockFetch(mockResponse, 200);
```

## Integration Testing

### Test Structure

Integration tests are located in the `tests/` directory and use Playwright to test full user flows with API mocking.

### Global API Mocking

Our integration tests include global API mocks that automatically intercept `/api/*` requests:

```typescript
// This is already set up in tests/test.ts
test.beforeEach(async ({ page }) => {
  // Mock /api/status endpoint
  await page.route('**/api/status', async route => {
    await route.fulfill({
      status: 200,
      contentType: 'application/json',
      body: JSON.stringify({
        status: 'ok',
        message: 'Server is running',
        timestamp: new Date().toISOString()
      })
    });
  });

  // Mock other API endpoints
  await page.route('**/api/**', async route => {
    const url = route.request().url();
    
    if (url.includes('/api/login')) {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          id: 1,
          display_name: 'Test User',
          email: 'test@example.com',
          createdAt: new Date().toISOString(),
          updatedAt: new Date().toISOString()
        })
      });
    } else {
      // Default mock for other endpoints
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ message: 'Mock response' })
      });
    }
  });
});
```

### Writing Integration Tests

```typescript
import { test, expect } from '@playwright/test';

test('user can login successfully', async ({ page }) => {
  await page.goto('/login');
  
  await page.getByLabel('Email').fill('test@example.com');
  await page.getByLabel('Password').fill('password123');
  await page.getByRole('button', { name: 'Login' }).click();
  
  // With API mocking, this will work even without a backend
  await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
});
```

### Testing Responsive Design

```typescript
test('page is responsive', async ({ page }) => {
  await page.goto('/');
  
  // Test desktop
  await page.setViewportSize({ width: 1200, height: 800 });
  await expect(page.getByRole('heading')).toBeVisible();
  
  // Test mobile
  await page.setViewportSize({ width: 375, height: 667 });
  await expect(page.getByRole('heading')).toBeVisible();
});
```

### Adding Custom API Mocks

For tests that need specific API responses:

```typescript
test('handles login error', async ({ page }) => {
  // Override the default login mock for this test
  await page.route('**/api/login', async route => {
    await route.fulfill({
      status: 401,
      contentType: 'application/json',
      body: JSON.stringify({ msg: 'Invalid credentials' })
    });
  });

  await page.goto('/login');
  // ... rest of test
});
```

## Test-Driven Development (TDD) Workflow

### 1. Write the Test First

```typescript
// UserProfile.test.ts
describe('UserProfile', () => {
  it('displays user information', () => {
    // Test will fail initially - that's expected!
    render(UserProfile, { props: { user: mockUser } });
    expect(screen.getByText('John Doe')).toBeInTheDocument();
  });
});
```

### 2. Run the Test

```bash
npm run test:unit:watch
```

### 3. Write the Implementation

```svelte
<!-- UserProfile.svelte -->
<script lang="ts">
  export let user: User;
</script>

<div>
  <h2>{user.display_name}</h2>
  <p>{user.email}</p>
</div>
```

### 4. Refactor

Once the test passes, refactor the code while keeping tests green.

## Best Practices

### Component Testing

1. **Test behavior, not implementation**: Focus on what the component does, not how it does it
2. **Use semantic queries**: Prefer `getByRole`, `getByLabelText` over `getByTestId`
3. **Test user interactions**: Click buttons, fill forms, test event handlers
4. **Test accessibility**: Ensure components are accessible by default
5. **Avoid testing CSS classes**: Svelte generates unique class names, so test functionality instead

### Store Testing

1. **Test state changes**: Verify store updates correctly
2. **Test subscriptions**: Ensure subscribers are notified of changes
3. **Mock dependencies**: Mock external dependencies like cookies, time functions
4. **Test initialization**: Verify store initializes correctly with different states

### Form Testing

1. **Test validation**: Verify form validation works correctly
2. **Test submission**: Test form submission with valid and invalid data
3. **Test error handling**: Ensure errors are displayed appropriately
4. **Test required fields**: Verify browser validation prevents submission when required fields are empty

### API Testing

1. **Mock responses**: Mock API responses for consistent testing
2. **Test loading states**: Verify loading indicators work correctly
3. **Test error states**: Ensure error handling works properly
4. **Use global mocks**: Set up common API mocks in `test.beforeEach`

## Debugging Tests

### Unit Tests

```bash
# Run specific test file
npm run test:unit -- ComponentName.test.ts

# Run tests with verbose output
npm run test:unit -- --reporter=verbose

# Debug failing tests
npm run test:unit:watch -- --reporter=verbose
```

### Integration Tests

```bash
# Run tests with UI
npm run test:integration:ui

# Run specific test
npm run test:integration -- --grep "test name"

# Debug with browser
npm run test:integration -- --headed --slowmo=1000
```

## Coverage

Generate coverage reports:

```bash
npm run test:unit:coverage
```

This will create coverage reports in multiple formats:
- Console output
- HTML report in `coverage/` directory
- JSON report for CI/CD integration

## Continuous Integration

The testing setup is designed to work with CI/CD pipelines:

```yaml
# Example GitHub Actions workflow
- name: Run Tests
  run: |
    npm install
    npm run test:unit:coverage
    npm run test:integration
```

## Troubleshooting

### Common Issues

1. **Import errors**: Ensure all dependencies are properly mocked
2. **Async test failures**: Use `waitFor` for async operations
3. **Component not rendering**: Check for missing props or dependencies
4. **Event handling issues**: Verify event dispatchers are properly set up
5. **SvelteKit reserved files**: Avoid `+` prefix in test filenames
6. **API mocking not working**: Check that route patterns match your API endpoints

### Getting Help

- Check the test utilities in `src/test/utils.ts`
- Review existing tests for patterns
- Use the test setup in `src/test/setup.ts` as reference
- Consult Vitest and Playwright documentation
- Check the global API mocks in `tests/test.ts` 