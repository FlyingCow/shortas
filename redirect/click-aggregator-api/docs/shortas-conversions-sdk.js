/**
 * Shortas Conversions SDK
 * 
 * A JavaScript SDK for tracking conversions in the Shortas URL shortening system.
 * This SDK provides easy-to-use methods for tracking various types of conversions
 * and automatically handles attribution to clicks.
 */

class ShortasConversions {
    constructor(config) {
        this.apiBaseUrl = config.apiBaseUrl || '/v1/conversions';
        this.authToken = config.authToken;
        this.defaultRouteId = config.routeId;
        this.defaultAttributionWindow = config.attributionWindow || 24; // hours
        this.sessionId = this.getOrCreateSessionId();
        this.userId = config.userId || this.getUserId();
        
        // Initialize click tracking if available
        this.clickId = this.getClickIdFromCookie();
        
        // Set up automatic page tracking
        if (config.autoTrackPageViews !== false) {
            this.trackPageView();
        }
    }

    /**
     * Track a conversion event
     * @param {Object} conversionData - Conversion data
     * @param {string} conversionData.type - Conversion type (purchase, signup, download, etc.)
     * @param {string} conversionData.name - Conversion name
     * @param {number} [conversionData.value] - Conversion value (for purchases)
     * @param {string} [conversionData.routeId] - Route ID (uses default if not provided)
     * @param {string} [conversionData.attributionType] - Attribution type
     * @param {Object} [conversionData.metadata] - Additional metadata
     */
    async trackConversion(conversionData) {
        const payload = {
            route_id: conversionData.routeId || this.defaultRouteId,
            conversion_type: conversionData.type,
            conversion_name: conversionData.name,
            conversion_value: conversionData.value || 0,
            attributed_click_id: this.clickId,
            attribution_type: conversionData.attributionType || 'direct',
            attribution_window_hours: this.defaultAttributionWindow,
            user_id: this.userId,
            session_id: this.sessionId,
            metadata: conversionData.metadata || {}
        };

        try {
            const response = await fetch(this.apiBaseUrl, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${this.authToken}`
                },
                body: JSON.stringify(payload)
            });

            if (!response.ok) {
                throw new Error(`Conversion tracking failed: ${response.statusText}`);
            }

            return await response.json();
        } catch (error) {
            console.error('Error tracking conversion:', error);
            throw error;
        }
    }

    /**
     * Track a purchase conversion
     * @param {Object} purchaseData - Purchase data
     * @param {number} purchaseData.value - Purchase amount
     * @param {string} purchaseData.name - Purchase name/description
     * @param {Object} [purchaseData.metadata] - Additional purchase metadata
     */
    async trackPurchase(purchaseData) {
        return this.trackConversion({
            type: 'purchase',
            name: purchaseData.name || 'Purchase',
            value: purchaseData.value,
            metadata: {
                product_id: purchaseData.productId,
                category: purchaseData.category,
                currency: purchaseData.currency || 'USD',
                ...purchaseData.metadata
            }
        });
    }

    /**
     * Track a signup conversion
     * @param {Object} signupData - Signup data
     * @param {string} signupData.name - Signup name/description
     * @param {Object} [signupData.metadata] - Additional signup metadata
     */
    async trackSignup(signupData) {
        return this.trackConversion({
            type: 'signup',
            name: signupData.name || 'User Signup',
            metadata: {
                plan: signupData.plan,
                source: signupData.source,
                ...signupData.metadata
            }
        });
    }

    /**
     * Track a download conversion
     * @param {Object} downloadData - Download data
     * @param {string} downloadData.name - Download name/description
     * @param {Object} [downloadData.metadata] - Additional download metadata
     */
    async trackDownload(downloadData) {
        return this.trackConversion({
            type: 'download',
            name: downloadData.name || 'File Download',
            metadata: {
                file_name: downloadData.fileName,
                file_type: downloadData.fileType,
                file_size: downloadData.fileSize,
                ...downloadData.metadata
            }
        });
    }

    /**
     * Track a form submission conversion
     * @param {Object} formData - Form data
     * @param {string} formData.name - Form name/description
     * @param {Object} [formData.metadata] - Additional form metadata
     */
    async trackFormSubmission(formData) {
        return this.trackConversion({
            type: 'form_submission',
            name: formData.name || 'Form Submission',
            metadata: {
                form_id: formData.formId,
                form_type: formData.formType,
                ...formData.metadata
            }
        });
    }

    /**
     * Track a custom conversion event
     * @param {Object} customData - Custom conversion data
     * @param {string} customData.type - Custom conversion type
     * @param {string} customData.name - Custom conversion name
     * @param {number} [customData.value] - Custom conversion value
     * @param {Object} [customData.metadata] - Additional metadata
     */
    async trackCustom(customData) {
        return this.trackConversion({
            type: `custom_${customData.type}`,
            name: customData.name,
            value: customData.value,
            metadata: customData.metadata || {}
        });
    }

    /**
     * Track funnel step completion
     * @param {Object} funnelData - Funnel data
     * @param {string} funnelData.funnelName - Name of the funnel
     * @param {string} funnelData.stepName - Name of the step
     * @param {number} funnelData.stepPosition - Position in funnel (1, 2, 3, etc.)
     * @param {number} [funnelData.stepValue] - Value at this step
     * @param {Object} [funnelData.metadata] - Additional metadata
     */
    async trackFunnelStep(funnelData) {
        const payload = {
            funnel_name: funnelData.funnelName,
            funnel_steps: funnelData.funnelSteps || [],
            route_id: funnelData.routeId || this.defaultRouteId,
            step_name: funnelData.stepName,
            step_position: funnelData.stepPosition,
            step_value: funnelData.stepValue || 0,
            user_id: this.userId,
            session_id: this.sessionId,
            metadata: funnelData.metadata || {}
        };

        try {
            const response = await fetch(`${this.apiBaseUrl}/funnels`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${this.authToken}`
                },
                body: JSON.stringify(payload)
            });

            if (!response.ok) {
                throw new Error(`Funnel tracking failed: ${response.statusText}`);
            }

            return await response.json();
        } catch (error) {
            console.error('Error tracking funnel step:', error);
            throw error;
        }
    }

    /**
     * Get conversion analytics
     * @param {Object} queryParams - Query parameters
     * @param {string} [queryParams.routeId] - Route ID filter
     * @param {string} [queryParams.conversionType] - Conversion type filter
     * @param {string} [queryParams.fromDate] - Start date (YYYY-MM-DD)
     * @param {string} [queryParams.toDate] - End date (YYYY-MM-DD)
     */
    async getAnalytics(queryParams = {}) {
        const params = new URLSearchParams();
        
        if (queryParams.routeId) params.append('route_id', queryParams.routeId);
        if (queryParams.conversionType) params.append('conversion_type', queryParams.conversionType);
        if (queryParams.fromDate) params.append('created_from', queryParams.fromDate);
        if (queryParams.toDate) params.append('created_to', queryParams.toDate);

        try {
            const response = await fetch(`${this.apiBaseUrl}?${params}`, {
                headers: {
                    'Authorization': `Bearer ${this.authToken}`
                }
            });

            if (!response.ok) {
                throw new Error(`Analytics fetch failed: ${response.statusText}`);
            }

            return await response.json();
        } catch (error) {
            console.error('Error fetching analytics:', error);
            throw error;
        }
    }

    /**
     * Get conversion summary for dashboard
     * @param {Object} queryParams - Query parameters
     */
    async getSummary(queryParams = {}) {
        const params = new URLSearchParams();
        
        if (queryParams.routeId) params.append('route_id', queryParams.routeId);
        if (queryParams.fromDate) params.append('from_date', queryParams.fromDate);
        if (queryParams.toDate) params.append('to_date', queryParams.toDate);

        try {
            const response = await fetch(`${this.apiBaseUrl}/summary?${params}`, {
                headers: {
                    'Authorization': `Bearer ${this.authToken}`
                }
            });

            if (!response.ok) {
                throw new Error(`Summary fetch failed: ${response.statusText}`);
            }

            return await response.json();
        } catch (error) {
            console.error('Error fetching summary:', error);
            throw error;
        }
    }

    /**
     * Get or create session ID
     */
    getOrCreateSessionId() {
        let sessionId = this.getCookie('shortas_session_id');
        if (!sessionId) {
            sessionId = this.generateId();
            this.setCookie('shortas_session_id', sessionId, 24); // 24 hours
        }
        return sessionId;
    }

    /**
     * Get user ID from cookie or generate one
     */
    getUserId() {
        let userId = this.getCookie('shortas_user_id');
        if (!userId) {
            userId = this.generateId();
            this.setCookie('shortas_user_id', userId, 365); // 1 year
        }
        return userId;
    }

    /**
     * Get click ID from cookie (set by click tracking)
     */
    getClickIdFromCookie() {
        return this.getCookie('shortas_click_id');
    }

    /**
     * Track page view (for session tracking)
     */
    trackPageView() {
        // This could be extended to track page views for session analysis
        console.log('Page view tracked for session:', this.sessionId);
    }

    /**
     * Generate a unique ID
     */
    generateId() {
        return 'conv_' + Date.now() + '_' + Math.random().toString(36).substr(2, 9);
    }

    /**
     * Get cookie value
     */
    getCookie(name) {
        const value = `; ${document.cookie}`;
        const parts = value.split(`; ${name}=`);
        if (parts.length === 2) return parts.pop().split(';').shift();
        return null;
    }

    /**
     * Set cookie
     */
    setCookie(name, value, hours) {
        const expires = new Date();
        expires.setTime(expires.getTime() + (hours * 60 * 60 * 1000));
        document.cookie = `${name}=${value};expires=${expires.toUTCString()};path=/`;
    }
}

// Export for use in modules
if (typeof module !== 'undefined' && module.exports) {
    module.exports = ShortasConversions;
}

// Global variable for browser use
if (typeof window !== 'undefined') {
    window.ShortasConversions = ShortasConversions;
}
