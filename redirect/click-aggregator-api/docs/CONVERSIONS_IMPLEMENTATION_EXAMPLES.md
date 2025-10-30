# Conversions Implementation Example

This example demonstrates how to implement the conversions functionality in a real-world e-commerce application.

## HTML Integration

```html
<!DOCTYPE html>
<html>
<head>
    <title>E-commerce Store</title>
    <script src="/js/shortas-conversions-sdk.js"></script>
</head>
<body>
    <!-- Product page -->
    <div id="product-page">
        <h1>Premium Headphones</h1>
        <p>Price: $199.99</p>
        <button id="add-to-cart">Add to Cart</button>
        <button id="buy-now">Buy Now</button>
    </div>

    <!-- Checkout page -->
    <div id="checkout-page" style="display: none;">
        <h2>Checkout</h2>
        <form id="checkout-form">
            <input type="email" id="email" placeholder="Email" required>
            <input type="text" id="name" placeholder="Name" required>
            <button type="submit">Complete Purchase</button>
        </form>
    </div>

    <script>
        // Initialize conversions tracking
        const conversions = new ShortasConversions({
            apiBaseUrl: '/v1/conversions',
            authToken: 'your-jwt-token',
            routeId: 'product-headphones-123',
            attributionWindow: 24,
            userId: 'user_456' // If user is logged in
        });

        // Track funnel steps
        let funnelSteps = [
            'product_view',
            'add_to_cart',
            'checkout_start',
            'checkout_complete',
            'purchase'
        ];

        // Track product view (funnel step 1)
        conversions.trackFunnelStep({
            funnelName: 'E-commerce Purchase Funnel',
            funnelSteps: funnelSteps,
            stepName: 'product_view',
            stepPosition: 1,
            metadata: {
                product_id: 'headphones-123',
                category: 'electronics',
                price: 199.99
            }
        });

        // Track add to cart (funnel step 2)
        document.getElementById('add-to-cart').addEventListener('click', async () => {
            await conversions.trackFunnelStep({
                funnelName: 'E-commerce Purchase Funnel',
                funnelSteps: funnelSteps,
                stepName: 'add_to_cart',
                stepPosition: 2,
                stepValue: 199.99,
                metadata: {
                    product_id: 'headphones-123',
                    quantity: 1
                }
            });
        });

        // Track checkout start (funnel step 3)
        document.getElementById('buy-now').addEventListener('click', async () => {
            await conversions.trackFunnelStep({
                funnelName: 'E-commerce Purchase Funnel',
                funnelSteps: funnelSteps,
                stepName: 'checkout_start',
                stepPosition: 3,
                stepValue: 199.99,
                metadata: {
                    product_id: 'headphones-123'
                }
            });
            
            // Show checkout form
            document.getElementById('product-page').style.display = 'none';
            document.getElementById('checkout-page').style.display = 'block';
        });

        // Track checkout complete and purchase (funnel steps 4 & 5)
        document.getElementById('checkout-form').addEventListener('submit', async (e) => {
            e.preventDefault();
            
            const email = document.getElementById('email').value;
            const name = document.getElementById('name').value;
            
            // Track checkout complete (funnel step 4)
            await conversions.trackFunnelStep({
                funnelName: 'E-commerce Purchase Funnel',
                funnelSteps: funnelSteps,
                stepName: 'checkout_complete',
                stepPosition: 4,
                stepValue: 199.99,
                metadata: {
                    email: email,
                    name: name
                }
            });
            
            // Track purchase conversion (funnel step 5)
            await conversions.trackPurchase({
                name: 'Premium Headphones Purchase',
                value: 199.99,
                productId: 'headphones-123',
                category: 'electronics',
                currency: 'USD',
                metadata: {
                    email: email,
                    name: name,
                    payment_method: 'credit_card'
                }
            });
            
            alert('Purchase completed! Thank you for your order.');
        });
    </script>
</body>
</html>
```

## React Integration

```jsx
import React, { useEffect, useState } from 'react';
import { ShortasConversions } from './shortas-conversions-sdk';

const ProductPage = ({ product, user }) => {
    const [conversions, setConversions] = useState(null);
    const [funnelStep, setFunnelStep] = useState(1);

    useEffect(() => {
        // Initialize conversions tracking
        const conversionsTracker = new ShortasConversions({
            apiBaseUrl: process.env.REACT_APP_CONVERSIONS_API_URL,
            authToken: user.token,
            routeId: product.routeId,
            userId: user.id
        });

        setConversions(conversionsTracker);

        // Track product view
        conversionsTracker.trackFunnelStep({
            funnelName: 'Product Purchase Funnel',
            funnelSteps: ['view', 'add_to_cart', 'checkout', 'purchase'],
            stepName: 'view',
            stepPosition: 1,
            metadata: {
                product_id: product.id,
                category: product.category,
                price: product.price
            }
        });
    }, [product, user]);

    const handleAddToCart = async () => {
        if (!conversions) return;

        await conversions.trackFunnelStep({
            funnelName: 'Product Purchase Funnel',
            funnelSteps: ['view', 'add_to_cart', 'checkout', 'purchase'],
            stepName: 'add_to_cart',
            stepPosition: 2,
            stepValue: product.price,
            metadata: {
                product_id: product.id,
                quantity: 1
            }
        });

        setFunnelStep(2);
        // Add to cart logic here
    };

    const handlePurchase = async () => {
        if (!conversions) return;

        await conversions.trackPurchase({
            name: `${product.name} Purchase`,
            value: product.price,
            productId: product.id,
            category: product.category,
            currency: 'USD',
            metadata: {
                user_id: user.id,
                payment_method: 'credit_card'
            }
        });

        setFunnelStep(4);
        // Purchase completion logic here
    };

    return (
        <div className="product-page">
            <h1>{product.name}</h1>
            <p>Price: ${product.price}</p>
            <p>Funnel Step: {funnelStep}/4</p>
            
            <button onClick={handleAddToCart}>
                Add to Cart
            </button>
            
            <button onClick={handlePurchase}>
                Buy Now
            </button>
        </div>
    );
};

export default ProductPage;
```

## Node.js Backend Integration

```javascript
const express = require('express');
const { ConversionStore } = require('./conversion-store');
const { Conversion, ConversionFunnel, ConversionGoal } = require('./conversion-models');

class EcommerceConversions {
    constructor(conversionStore) {
        this.conversionStore = conversionStore;
    }

    // Track purchase conversion
    async trackPurchase(purchaseData) {
        const conversion = new Conversion({
            id: this.generateId(),
            owner_id: purchaseData.ownerId,
            creator_id: purchaseData.creatorId,
            route_id: purchaseData.routeId,
            workspace_id: purchaseData.workspaceId,
            conversion_type: 'purchase',
            conversion_name: purchaseData.name,
            conversion_value: purchaseData.value,
            attributed_click_id: purchaseData.clickId,
            attribution_type: 'direct',
            user_id: purchaseData.userId,
            session_id: purchaseData.sessionId,
            metadata: JSON.stringify({
                product_id: purchaseData.productId,
                category: purchaseData.category,
                currency: purchaseData.currency,
                payment_method: purchaseData.paymentMethod
            }),
            created: new Date(),
            click_created: purchaseData.clickCreated || new Date()
        });

        await this.conversionStore.store_conversion(conversion);
        return conversion;
    }

    // Track funnel step
    async trackFunnelStep(funnelData) {
        const funnel = new ConversionFunnel({
            id: this.generateId(),
            owner_id: funnelData.ownerId,
            workspace_id: funnelData.workspaceId,
            funnel_name: funnelData.funnelName,
            funnel_steps: funnelData.funnelSteps,
            user_id: funnelData.userId,
            session_id: funnelData.sessionId,
            route_id: funnelData.routeId,
            step_name: funnelData.stepName,
            step_position: funnelData.stepPosition,
            step_completed: 1,
            step_value: funnelData.stepValue || 0,
            step_created: new Date(),
            funnel_started: funnelData.funnelStarted || new Date(),
            metadata: JSON.stringify(funnelData.metadata || {})
        });

        await this.conversionStore.store_conversion_funnel(funnel);
        return funnel;
    }

    // Create conversion goal
    async createConversionGoal(goalData) {
        const goal = new ConversionGoal({
            id: this.generateId(),
            owner_id: goalData.ownerId,
            workspace_id: goalData.workspaceId,
            route_id: goalData.routeId,
            goal_name: goalData.name,
            goal_type: goalData.type,
            target_value: goalData.targetValue,
            target_period: goalData.period,
            attribution_window_hours: goalData.attributionWindow || 24,
            is_active: 1,
            created: new Date(),
            updated: new Date()
        });

        await this.conversionStore.store_conversion_goal(goal);
        return goal;
    }

    // Get conversion analytics
    async getConversionAnalytics(ownerId, routeId, fromDate, toDate) {
        const rates = await this.conversionStore.get_conversion_rates(
            ownerId, routeId, fromDate, toDate
        );

        const revenue = await this.conversionStore.get_revenue_analytics(
            ownerId, routeId, fromDate, toDate
        );

        const attribution = await this.conversionStore.get_conversion_attribution_analysis(
            ownerId, routeId, fromDate, toDate
        );

        return {
            conversionRates: rates,
            revenueAnalytics: revenue,
            attributionAnalysis: attribution
        };
    }

    generateId() {
        return 'conv_' + Date.now() + '_' + Math.random().toString(36).substr(2, 9);
    }
}

// Express.js API endpoints
const app = express();
app.use(express.json());

const conversionStore = new ConversionStore(); // Your ClickHouse implementation
const conversions = new EcommerceConversions(conversionStore);

// Track conversion endpoint
app.post('/api/conversions', async (req, res) => {
    try {
        const { type, data } = req.body;
        
        let result;
        switch (type) {
            case 'purchase':
                result = await conversions.trackPurchase(data);
                break;
            case 'funnel_step':
                result = await conversions.trackFunnelStep(data);
                break;
            case 'goal':
                result = await conversions.createConversionGoal(data);
                break;
            default:
                throw new Error('Unknown conversion type');
        }

        res.status(201).json({ success: true, data: result });
    } catch (error) {
        res.status(500).json({ success: false, error: error.message });
    }
});

// Get analytics endpoint
app.get('/api/conversions/analytics', async (req, res) => {
    try {
        const { owner_id, route_id, from_date, to_date } = req.query;
        
        const analytics = await conversions.getConversionAnalytics(
            owner_id, route_id, from_date, to_date
        );

        res.json({ success: true, data: analytics });
    } catch (error) {
        res.status(500).json({ success: false, error: error.message });
    }
});

app.listen(3000, () => {
    console.log('Conversions API server running on port 3000');
});
```

## Python Integration

```python
import requests
import json
from datetime import datetime
from typing import Optional, Dict, Any

class ShortasConversions:
    def __init__(self, api_base_url: str, auth_token: str, route_id: str):
        self.api_base_url = api_base_url
        self.auth_token = auth_token
        self.route_id = route_id
        self.session_id = self._get_or_create_session_id()
        self.user_id = self._get_user_id()

    def track_conversion(self, conversion_type: str, name: str, 
                        value: Optional[float] = None, 
                        metadata: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
        """Track a conversion event"""
        payload = {
            "route_id": self.route_id,
            "conversion_type": conversion_type,
            "conversion_name": name,
            "conversion_value": value or 0,
            "attributed_click_id": self._get_click_id(),
            "attribution_type": "direct",
            "attribution_window_hours": 24,
            "user_id": self.user_id,
            "session_id": self.session_id,
            "metadata": metadata or {}
        }

        response = requests.post(
            f"{self.api_base_url}/conversions",
            headers={
                "Content-Type": "application/json",
                "Authorization": f"Bearer {self.auth_token}"
            },
            json=payload
        )

        response.raise_for_status()
        return response.json()

    def track_purchase(self, name: str, value: float, 
                      product_id: str, category: str,
                      metadata: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
        """Track a purchase conversion"""
        purchase_metadata = {
            "product_id": product_id,
            "category": category,
            "currency": "USD",
            **(metadata or {})
        }

        return self.track_conversion(
            conversion_type="purchase",
            name=name,
            value=value,
            metadata=purchase_metadata
        )

    def track_signup(self, name: str, plan: str = None,
                    metadata: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
        """Track a signup conversion"""
        signup_metadata = {
            "plan": plan,
            "source": "website",
            **(metadata or {})
        }

        return self.track_conversion(
            conversion_type="signup",
            name=name,
            metadata=signup_metadata
        )

    def track_funnel_step(self, funnel_name: str, step_name: str,
                          step_position: int, step_value: float = 0,
                          metadata: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
        """Track a funnel step completion"""
        payload = {
            "funnel_name": funnel_name,
            "funnel_steps": [],
            "route_id": self.route_id,
            "step_name": step_name,
            "step_position": step_position,
            "step_value": step_value,
            "user_id": self.user_id,
            "session_id": self.session_id,
            "metadata": metadata or {}
        }

        response = requests.post(
            f"{self.api_base_url}/conversions/funnels",
            headers={
                "Content-Type": "application/json",
                "Authorization": f"Bearer {self.auth_token}"
            },
            json=payload
        )

        response.raise_for_status()
        return response.json()

    def get_analytics(self, route_id: str = None, conversion_type: str = None,
                     from_date: str = None, to_date: str = None) -> Dict[str, Any]:
        """Get conversion analytics"""
        params = {}
        if route_id:
            params["route_id"] = route_id
        if conversion_type:
            params["conversion_type"] = conversion_type
        if from_date:
            params["created_from"] = from_date
        if to_date:
            params["created_to"] = to_date

        response = requests.get(
            f"{self.api_base_url}/conversions",
            headers={"Authorization": f"Bearer {self.auth_token}"},
            params=params
        )

        response.raise_for_status()
        return response.json()

    def get_summary(self, route_id: str = None, 
                   from_date: str = None, to_date: str = None) -> Dict[str, Any]:
        """Get conversion summary for dashboard"""
        params = {}
        if route_id:
            params["route_id"] = route_id
        if from_date:
            params["from_date"] = from_date
        if to_date:
            params["to_date"] = to_date

        response = requests.get(
            f"{self.api_base_url}/conversions/summary",
            headers={"Authorization": f"Bearer {self.auth_token}"},
            params=params
        )

        response.raise_for_status()
        return response.json()

    def _get_or_create_session_id(self) -> str:
        """Get or create session ID"""
        # In a real implementation, you'd store this in a session store
        return f"session_{datetime.now().timestamp()}"

    def _get_user_id(self) -> str:
        """Get user ID"""
        # In a real implementation, you'd get this from authentication
        return f"user_{datetime.now().timestamp()}"

    def _get_click_id(self) -> str:
        """Get click ID from session/cookie"""
        # In a real implementation, you'd get this from the click tracking system
        return f"click_{datetime.now().timestamp()}"

# Usage example
if __name__ == "__main__":
    conversions = ShortasConversions(
        api_base_url="https://api.shortas.com/v1",
        auth_token="your-jwt-token",
        route_id="product-page-123"
    )

    # Track a purchase
    result = conversions.track_purchase(
        name="Premium Headphones Purchase",
        value=199.99,
        product_id="headphones-123",
        category="electronics",
        metadata={"payment_method": "credit_card"}
    )
    print(f"Purchase tracked: {result}")

    # Track funnel steps
    conversions.track_funnel_step(
        funnel_name="E-commerce Purchase Funnel",
        step_name="product_view",
        step_position=1,
        metadata={"product_id": "headphones-123"}
    )

    # Get analytics
    analytics = conversions.get_analytics(
        route_id="product-page-123",
        from_date="2024-01-01",
        to_date="2024-01-31"
    )
    print(f"Analytics: {analytics}")
```

## Summary

This comprehensive conversions functionality provides:

1. **Complete Database Schema** - ClickHouse tables and materialized views for fast analytics
2. **REST API Endpoints** - Full CRUD operations for conversions, goals, and funnels
3. **JavaScript SDK** - Easy-to-use client-side tracking
4. **Multiple Language Support** - Examples in JavaScript, React, Node.js, and Python
5. **Advanced Analytics** - Conversion rates, attribution analysis, funnel performance, and ROI metrics
6. **Flexible Attribution** - Support for direct, session, time-based, and multi-touch attribution
7. **Real-time Processing** - Fast conversion tracking and analytics
8. **Comprehensive Documentation** - Complete implementation guide and examples

The system integrates seamlessly with your existing Shortas architecture and provides enterprise-grade conversion tracking capabilities.
