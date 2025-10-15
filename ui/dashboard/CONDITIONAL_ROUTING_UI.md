# Conditional Routing Dashboard UI

The dashboard now supports creating and managing conditional routing policies through an intuitive visual interface.

## Features Added

### 1. **TypeScript Type Definitions** (`src/services/api.ts`)

Added comprehensive TypeScript interfaces for conditional routing:

```typescript
// Expression for condition matching
export interface Expression {
  default_operator?: 'And' | 'Or';
  ua?: StringCondition;        // User Agent
  os?: StringCondition;         // Operating System
  device?: StringCondition;     // Device Type
  lang?: StringCondition;       // Language
  country?: StringCondition;    // Country
  date?: DateCondition;         // Date
  rnd?: NumericCondition;       // Random (A/B testing)
  day_of_week?: NumericCondition;
  day_of_month?: NumericCondition;
  month?: NumericCondition;
  and?: Expression[];           // Nested AND conditions
  or?: Expression[];            // Nested OR conditions
}

// Routing policy types
export type RoutingPolicy =
  | 'Basic'
  | 'Mirroring'
  | { Conditional: ConditionalRouting[] }
  | { Challenge: ChallengeRouting }
  | { File: FileRouting };
```

### 2. **PolicyEditor Component** (`src/components/PolicyEditor.tsx`)

Interactive editor for managing routing policies with:

- **Policy Type Selection**: Choose between Basic, Conditional, Challenge, File, or Mirroring
- **Condition Management**: Add, edit, and remove conditions
- **Route Key Assignment**: Specify which route to redirect to when conditions match
- **Condition Builder**: Visual interface for building complex conditions

**Features:**
- Collapsible UI to save space
- Add multiple conditions per route
- Support for all condition types (UA, OS, Device, Country, Date, etc.)
- Dynamic operator selection (eq, in, starts, ends, gt, lt)
- Real-time validation

### 3. **RouteFormModal Component** (`src/components/RouteFormModal.tsx`)

Comprehensive modal for creating/editing routes:

**Sections:**
1. **Basic Information**
   - Short Link
   - Destination URL
   - Switch
   - Status
   - HTTP Code
   - TTL
   - Terminal

2. **Routing Policy** (with PolicyEditor)
   - Full conditional routing support
   - Visual policy configuration

3. **Properties**
   - Route ID
   - Domain ID
   - Tags
   - OpenGraph toggle
   - Allow Debug toggle

### 4. **Routes Table Enhancement** (`src/components/RoutesUnified.tsx`)

Added "Policy" column to routes table showing:
- **Basic** - Standard redirect (default)
- **Conditional** - Conditional routing (blue badge)
- **Challenge** - Challenge page (yellow badge)
- **File** - File serving (gray badge)
- **Mirroring** - Mirroring (blue badge)

## Usage Guide

### Creating a Basic Route

1. Click "Create Route" button
2. Fill in Short Link and Destination URL
3. Leave Policy as "Basic"
4. Click "Create Route"

### Creating a Conditional Route

1. Click "Create Route" button
2. Fill in basic information
3. Select "Conditional" from Policy dropdown
4. Click "Expand" to show policy editor
5. Click "Add Condition" to add routing rules

#### Example: Mobile vs Desktop Routing

```
Condition 1:
  Route Key: mobile-landing
  Condition:
    - Device: in [Mobile, Smartphone]

Condition 2:
  Route Key: desktop-landing
  Condition:
    - Device: eq Desktop
```

When users visit the short link:
- Mobile users → redirected to route with key "mobile-landing"
- Desktop users → redirected to route with key "desktop-landing"
- Others → redirected to default destination

#### Example: Geographic Targeting

```
Condition 1:
  Route Key: us-page
  Condition:
    - Country: eq US

Condition 2:
  Route Key: eu-page
  Condition:
    - Country: in [UK, FR, DE, IT, ES]
```

#### Example: Browser-Specific Routing

```
Condition 1:
  Route Key: chrome-optimized
  Condition:
    - UA: in [Chrome, Chromium, Edge]

Condition 2:
  Route Key: firefox-optimized
  Condition:
    - UA: eq Firefox
```

#### Example: A/B Testing

```
Condition 1:
  Route Key: variant-a
  Condition:
    - RND: lt 50

Condition 2:
  Route Key: variant-b
  Condition:
    - RND: gte 50
```

Splits traffic 50/50 between two variants.

### Editing Routes

1. Click the edit icon (pencil) on any route
2. Modify any field including routing policy
3. Add/remove/edit conditions
4. Click "Update Route"

### Viewing Policy Information

- Routes table displays policy type in "Policy" column
- Color-coded badges for quick identification:
  - **Gray** - Basic/File
  - **Blue** - Conditional/Mirroring
  - **Yellow** - Challenge

## Condition Types Reference

### String Conditions (UA, OS, Device, Country, Lang)

- **eq** - Exact match
  ```
  UA: eq "Chrome"
  ```

- **in** - Match any in list
  ```
  Country: in ["US", "CA", "UK"]
  ```

- **starts** - Starts with
  ```
  OS: starts "Windows"
  ```

- **ends** - Ends with
  ```
  Device: ends "Phone"
  ```

### Numeric Conditions (Day of Week, Day of Month, Month, RND)

- **eq** - Equals
  ```
  Day of Week: eq 5 (Friday)
  ```

- **gt** - Greater than
  ```
  Day of Month: gt 15 (after 15th)
  ```

- **lt** - Less than
  ```
  RND: lt 50 (50% traffic)
  ```

- **in** - In list
  ```
  Month: in [6, 7, 8] (Summer months)
  ```

### Date Conditions

- **eq** - Specific date
  ```
  Date: eq "2025-12-25"
  ```

- **gt** - After date
  ```
  Date: gt "2025-01-01"
  ```

- **lt** - Before date
  ```
  Date: lt "2025-12-31"
  ```

- **in** - List of dates
  ```
  Date: in ["2025-12-25", "2025-01-01"]
  ```

## Advanced Features

### Multiple Conditions

You can add multiple condition types to a single condition:

```
Route Key: premium-users
Conditions:
  - Country: eq "US"
  - Device: eq "Mobile"
  - Day of Week: in [1, 2, 3, 4, 5] (Weekdays)
```

All conditions must match (AND logic by default).

### Adding Operators

Each condition type supports adding multiple operators:

```
Country Condition:
  - eq: "US"        (if exact match)
  - in: ["CA", "MX"] (OR match these)
```

Click "+ Add operator" to add more operators to a condition.

## API Integration

The UI sends routing policies in the format expected by the C# Proxy API:

```json
{
  "switch": "main",
  "link": "example.com/promo",
  "dest": "https://default.com",
  "policy": {
    "Conditional": [
      {
        "key": "mobile-users",
        "condition": {
          "device": {
            "in": ["Mobile", "Smartphone"]
          }
        }
      }
    ]
  }
}
```

## Browser Compatibility

- Modern browsers (Chrome, Firefox, Safari, Edge)
- Responsive design works on desktop and tablet
- Requires JavaScript enabled

## Performance

- Lazy loading of policy editor (only loads when expanded)
- Efficient re-rendering with React hooks
- No unnecessary API calls

## Troubleshooting

### "Policy not saving"
- Ensure all condition keys are filled in
- Check that operators have values
- Verify destination URL is valid

### "Conditions not appearing"
- Click "Expand" button on Policy section
- Ensure "Conditional" policy type is selected
- Check browser console for errors

### "Modal not closing"
- Click the X button or "Cancel" button
- Check for validation errors

## Future Enhancements

Potential improvements for future versions:
- Condition templates for common use cases
- Visual condition flow diagram
- Test condition matching with sample data
- Import/export policy configurations
- Policy validation and warnings
- Condition statistics and analytics

## Related Documentation

- [Backend Conditional Routing](../../api/CONDITIONAL_ROUTING.md)
- [Click-Router Documentation](../../redirect/click-router/docs/)
- [API Integration Guide](./ROUTES_INTEGRATION.md)
