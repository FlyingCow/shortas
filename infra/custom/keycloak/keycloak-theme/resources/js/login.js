document.addEventListener('DOMContentLoaded', function () {
  var form = document.getElementById('kc-form-login');
  var btn = document.getElementById('kc-login');

  // Loading state on submit
  if (form && btn) {
    form.addEventListener('submit', function () {
      btn.disabled = true;
      btn.classList.add('shortas-btn-loading');
      btn.innerHTML = '<span class="shortas-btn-spinner"></span> Signing in\u2026';
    });
  }

  // Auto-focus first empty field
  var username = document.getElementById('username');
  var password = document.getElementById('password');

  if (username && !username.value) {
    username.focus();
  } else if (password) {
    password.focus();
  }

  // Inline validation — mark empty required fields on blur
  var inputs = document.querySelectorAll('.shortas-form-input');
  inputs.forEach(function (input) {
    input.addEventListener('blur', function () {
      if (this.value.trim() === '') {
        this.classList.add('is-invalid');
      } else {
        this.classList.remove('is-invalid');
      }
    });

    // Clear error state on input
    input.addEventListener('input', function () {
      this.classList.remove('is-invalid');
    });
  });
});
