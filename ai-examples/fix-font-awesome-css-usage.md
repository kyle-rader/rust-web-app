# Font Awesome Setup in SvelteKit

## Problem
Setting up Font Awesome webfonts in a SvelteKit project where the dev server wasn't finding the webfonts when loaded from the `src/static` directory.

## Solution

### 1. Directory Structure
The key was to move static assets from `src/static` to a root-level `static` directory. The correct structure should be:

```
client/
  ├── static/
  │   ├── webfonts/
  │   │   ├── fa-solid-900.woff2
  │   │   ├── fa-regular-400.woff2
  │   │   └── ... (other font files)
  │   └── all.min.css
  └── src/
      └── ...
```

### 2. CSS Import Update
Updated the Font Awesome CSS import in `app.less` to use the correct static path:

```less
@import 'reset';
@import 'colors';
@import 'responsive';
@import '/all.min.css';  // Changed from '../static/all.min.css'
```

### 3. Why This Works
In SvelteKit:
- Files in the `static` directory at the root of your project are served at the root path (`/`)
- The `src/static` directory is not automatically served by SvelteKit
- When importing files from the static directory in CSS, use a path relative to the root (`/`), not relative to source files

### 4. Verification
You can verify the setup is working by:
1. Running the dev server
2. Opening browser's developer tools
3. Going to the Network tab
4. Refreshing the page
5. Looking for requests to `/webfonts/` and `/all.min.css`

## Commands Used
```bash
# Create static directory structure
mkdir -p client/static/webfonts

# Move Font Awesome files to correct location
mv client/src/static/webfonts/* client/static/webfonts/
mv client/src/static/all.min.css client/static/
``` 