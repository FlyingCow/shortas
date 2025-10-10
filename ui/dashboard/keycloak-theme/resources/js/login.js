// Keycloak Login Theme JavaScript
document.addEventListener('DOMContentLoaded', function() {
    // Add loading state to login button
    const loginForm = document.getElementById('kc-form-login');
    const loginButton = document.getElementById('kc-login');
    
    if (loginForm && loginButton) {
        loginForm.addEventListener('submit', function() {
            loginButton.disabled = true;
            loginButton.innerHTML = '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M8 12l2 2 4-4"/></svg> Signing In...';
        });
    }
    
    // Add focus management
    const usernameInput = document.getElementById('username');
    const passwordInput = document.getElementById('password');
    
    if (usernameInput && !usernameInput.value) {
        usernameInput.focus();
    } else if (passwordInput) {
        passwordInput.focus();
    }
    
    // Add form validation
    const inputs = document.querySelectorAll('.keycloak-login-form-input');
    inputs.forEach(input => {
        input.addEventListener('blur', function() {
            if (this.value.trim() === '') {
                this.style.borderColor = '#ef4444';
            } else {
                this.style.borderColor = '#e5e7eb';
            }
        });
    });
    
    // Add smooth transitions
    const features = document.querySelectorAll('.keycloak-login-feature');
    features.forEach(feature => {
        feature.addEventListener('mouseenter', function() {
            this.style.transform = 'translateX(5px)';
        });
        
        feature.addEventListener('mouseleave', function() {
            this.style.transform = 'translateX(0)';
        });
    });
    
    // Add button hover effects
    const buttons = document.querySelectorAll('.keycloak-login-button');
    buttons.forEach(button => {
        button.addEventListener('mouseenter', function() {
            this.style.transform = 'translateY(-2px)';
        });
        
        button.addEventListener('mouseleave', function() {
            this.style.transform = 'translateY(0)';
        });
    });
});

