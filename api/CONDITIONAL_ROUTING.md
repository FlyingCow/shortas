# Conditional Routing

The C# Proxy API now supports conditional routing, matching the click-router's powerful routing policy system.

## Overview

Conditional routing allows routes to redirect to different destinations based on dynamic conditions such as:
- User Agent (browser)
- Operating System
- Device type (mobile, tablet, desktop)
- Geographic location (country)
- Language
- Date and time constraints
- Random distribution (A/B testing)

## Routing Policies

### Policy Types

The API supports five routing policy types:

1. **Basic** - Simple redirect to `dest` field (default)
2. **Conditional** - Redirect based on conditions
3. **Challenge** - Show a challenge page before redirecting
4. **File** - Serve a file instead of redirecting
5. **Mirroring** - Mirror the destination website

## Conditional Policy Structure

### Example: Browser-Based Routing

```json
{
  "switch": "main",
  "link": "example.com/mylink",
  "dest": "https://default.com",
  "policy": {
    "Conditional": [
      {
        "key": "chrome-users",
        "condition": {
          "ua": {
            "in": ["Chrome", "Chromium", "Edge"]
          }
        }
      },
      {
        "key": "firefox-users",
        "condition": {
          "ua": {
            "eq": "Firefox"
          }
        }
      }
    ]
  }
}
```

When a user clicks this link:
- If using Chrome, Edge, or Chromium → redirects to route with key "chrome-users"
- If using Firefox → redirects to route with key "firefox-users"
- Otherwise → redirects to `dest` (https://default.com)

### Example: Device-Based Routing

```json
{
  "policy": {
    "Conditional": [
      {
        "key": "mobile-page",
        "condition": {
          "device": {
            "in": ["Mobile", "Smartphone"]
          }
        }
      },
      {
        "key": "desktop-page",
        "condition": {
          "device": {
            "eq": "Desktop"
          }
        }
      }
    ]
  }
}
```

### Example: Geographic Routing

```json
{
  "policy": {
    "Conditional": [
      {
        "key": "us-page",
        "condition": {
          "country": {
            "eq": "US"
          }
        }
      },
      {
        "key": "eu-page",
        "condition": {
          "country": {
            "in": ["DE", "FR", "IT", "ES", "UK"]
          }
        }
      }
    ]
  }
}
```

### Example: Time-Based Routing

```json
{
  "policy": {
    "Conditional": [
      {
        "key": "weekend-special",
        "condition": {
          "day_of_week": {
            "in": [6, 7]
          }
        }
      },
      {
        "key": "month-end",
        "condition": {
          "day_of_month": {
            "gt": 25
          }
        }
      }
    ]
  }
}
```

### Example: Complex Conditions with AND/OR

```json
{
  "policy": {
    "Conditional": [
      {
        "key": "mobile-chrome-us",
        "condition": {
          "default_operator": "And",
          "device": {
            "eq": "Mobile"
          },
          "ua": {
            "in": ["Chrome", "Edge"]
          },
          "country": {
            "eq": "US"
          }
        }
      },
      {
        "key": "weekend-or-holiday",
        "condition": {
          "or": [
            {
              "day_of_week": {
                "in": [6, 7]
              }
            },
            {
              "date": {
                "in": ["2025-12-25", "2025-01-01"]
              }
            }
          ]
        }
      }
    ]
  }
}
```

### Example: A/B Testing with Random Distribution

```json
{
  "policy": {
    "Conditional": [
      {
        "key": "variant-a",
        "condition": {
          "rnd": {
            "lt": 50
          }
        }
      },
      {
        "key": "variant-b",
        "condition": {
          "rnd": {
            "gte": 50
          }
        }
      }
    ]
  }
}
```

## Condition Operators

### String Conditions (UA, OS, Device, Lang, Country)

- **eq**: Equals (exact match)
  ```json
  { "ua": { "eq": "Chrome" } }
  ```

- **in**: In list (any match)
  ```json
  { "country": { "in": ["US", "CA", "UK"] } }
  ```

- **starts**: Starts with
  ```json
  { "os": { "starts": "Windows" } }
  ```

- **ends**: Ends with
  ```json
  { "device": { "ends": "Phone" } }
  ```

### Numeric Conditions (DayOfMonth, DayOfWeek, Month, RND)

- **eq**: Equals
  ```json
  { "day_of_week": { "eq": 5 } }
  ```

- **gt**: Greater than
  ```json
  { "day_of_month": { "gt": 15 } }
  ```

- **lt**: Less than
  ```json
  { "rnd": { "lt": 50 } }
  ```

- **in**: In list
  ```json
  { "month": { "in": [6, 7, 8] } }
  ```

### Date Conditions

- **eq**: Equals specific date
  ```json
  { "date": { "eq": "2025-12-25" } }
  ```

- **gt**: After date
  ```json
  { "date": { "gt": "2025-01-01" } }
  ```

- **lt**: Before date
  ```json
  { "date": { "lt": "2025-12-31" } }
  ```

- **in**: In date list
  ```json
  { "date": { "in": ["2025-12-25", "2025-01-01"] } }
  ```

## Logical Operators

### AND (default)

```json
{
  "condition": {
    "default_operator": "And",
    "device": { "eq": "Mobile" },
    "country": { "eq": "US" }
  }
}
```

### OR

```json
{
  "condition": {
    "or": [
      { "device": { "eq": "Mobile" } },
      { "device": { "eq": "Tablet" } }
    ]
  }
}
```

### Nested AND/OR

```json
{
  "condition": {
    "and": [
      { "country": { "eq": "US" } },
      {
        "or": [
          { "device": { "eq": "Mobile" } },
          { "device": { "eq": "Tablet" } }
        ]
      }
    ]
  }
}
```

## API Usage

### Create Route with Conditional Policy

```bash
POST /api/v1/routes
Content-Type: application/json

{
  "switch": "main",
  "link": "example.com/campaign",
  "dest": "https://default.com/landing",
  "destFormat": "Http",
  "status": "Active",
  "terminal": "External",
  "policy": {
    "Conditional": [
      {
        "key": "mobile-landing",
        "condition": {
          "device": { "in": ["Mobile", "Smartphone"] }
        }
      }
    ]
  },
  "properties": {
    "routeId": "campaign-001",
    "ownerId": "user-123"
  }
}
```

### Update Route Policy

```bash
PUT /api/v1/routes/example.com/campaign
Content-Type: application/json

{
  "policy": {
    "Conditional": [
      {
        "key": "ios-landing",
        "condition": {
          "os": { "eq": "iOS" }
        }
      },
      {
        "key": "android-landing",
        "condition": {
          "os": { "eq": "Android" }
        }
      }
    ]
  }
}
```

## Database Storage

Policies are stored as JSONB in PostgreSQL, allowing:
- Efficient querying
- Flexible schema evolution
- Index support for better performance

The `PolicyJson` field stores the serialized policy:
```sql
{
  "Conditional": [
    {
      "key": "mobile",
      "condition": {
        "device": { "in": ["Mobile"] }
      }
    }
  ]
}
```

## Benefits

1. **Dynamic Routing**: Route users to different destinations without changing URLs
2. **Personalization**: Serve content based on user attributes
3. **A/B Testing**: Split traffic for experimentation
4. **Geo-Targeting**: Show location-specific content
5. **Device Optimization**: Provide device-optimized experiences
6. **Time-Based Campaigns**: Activate/deactivate routes based on time

## Complete Example: Marketing Campaign

```json
{
  "switch": "main",
  "link": "example.com/promo",
  "dest": "https://example.com/default",
  "policy": {
    "Conditional": [
      {
        "key": "us-mobile-chrome",
        "condition": {
          "default_operator": "And",
          "country": { "eq": "US" },
          "device": { "eq": "Mobile" },
          "ua": { "in": ["Chrome", "Edge"] },
          "day_of_week": { "in": [1, 2, 3, 4, 5] }
        }
      },
      {
        "key": "weekend-special",
        "condition": {
          "day_of_week": { "in": [6, 7] }
        }
      },
      {
        "key": "international",
        "condition": {
          "country": { "in": ["UK", "CA", "AU"] }
        }
      }
    ]
  },
  "properties": {
    "routeId": "promo-2025-q1",
    "tags": ["marketing", "campaign", "promo"]
  }
}
```

This routes users to:
- `us-mobile-chrome` route: US mobile Chrome users on weekdays
- `weekend-special` route: Anyone on weekends
- `international` route: Users from UK, Canada, or Australia
- Default `dest`: Everyone else
