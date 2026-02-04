import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Check, ChevronLeft, ChevronRight, Rocket, Globe, CheckCircle } from 'lucide-react';
import { apiService, CreateDomainDto } from '../services/api';
import LoadingSpinner from './LoadingSpinner';
import './DesignSystem.css';

const wizardStyles = `
.wizard-container {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, var(--primary) 0%, var(--secondary) 100%);
  padding: 2rem;
}

.wizard-card {
  background: var(--bg-elevated);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-2xl);
  width: 100%;
  max-width: 700px;
  overflow: hidden;
}

.wizard-header {
  padding: 2rem;
  text-align: center;
  border-bottom: 1px solid var(--border-primary);
}

.wizard-header h1 {
  margin: 0 0 0.5rem 0;
  font-size: var(--font-size-2xl);
  font-weight: var(--font-weight-bold);
  color: var(--text-primary);
}

.wizard-header p {
  margin: 0;
  color: var(--text-secondary);
  font-size: var(--font-size-md);
}

.wizard-progress {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 2rem;
  gap: 1rem;
}

.wizard-step {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.wizard-step-number {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: var(--font-weight-semibold);
  transition: all var(--transition-normal);
  border: 2px solid var(--border-secondary);
  background: var(--bg-secondary);
  color: var(--text-tertiary);
}

.wizard-step.active .wizard-step-number {
  background: linear-gradient(135deg, var(--primary) 0%, var(--secondary) 100%);
  border-color: var(--primary);
  color: white;
  box-shadow: 0 0 0 4px var(--color-primary-alpha);
}

.wizard-step.completed .wizard-step-number {
  background: var(--success);
  border-color: var(--success);
  color: white;
}

.wizard-step-label {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-tertiary);
}

.wizard-step.active .wizard-step-label {
  color: var(--text-primary);
}

.wizard-step.completed .wizard-step-label {
  color: var(--text-secondary);
}

.wizard-divider {
  width: 40px;
  height: 2px;
  background: var(--border-secondary);
  transition: background var(--transition-normal);
}

.wizard-step.completed ~ .wizard-step .wizard-divider {
  background: var(--success);
}

.wizard-body {
  padding: 2rem;
  min-height: 300px;
}

.wizard-step-content {
  animation: fadeIn 0.3s ease-in-out;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.wizard-step-title {
  font-size: var(--font-size-xl);
  font-weight: var(--font-weight-semibold);
  margin: 0 0 0.5rem 0;
  color: var(--text-primary);
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.wizard-step-description {
  color: var(--text-secondary);
  margin: 0 0 2rem 0;
  font-size: var(--font-size-md);
}

.wizard-footer {
  padding: 1.5rem 2rem;
  border-top: 1px solid var(--border-primary);
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 1rem;
}

.wizard-success {
  text-align: center;
  padding: 2rem;
}

.wizard-success-icon {
  width: 80px;
  height: 80px;
  margin: 0 auto 1.5rem;
  color: var(--success);
  animation: scaleIn 0.5s ease-in-out;
}

@keyframes scaleIn {
  from {
    opacity: 0;
    transform: scale(0.5);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

.wizard-success h2 {
  margin: 0 0 0.5rem 0;
  color: var(--text-primary);
}

.wizard-success p {
  margin: 0 0 2rem 0;
  color: var(--text-secondary);
}

.form-group {
  margin-bottom: 1.5rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  font-size: var(--font-size-sm);
}

.form-group input,
.form-group select {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid var(--border-primary);
  border-radius: 0;
  background: var(--bg-secondary);
  color: var(--text-primary);
  font-size: var(--font-size-md);
  transition: all var(--transition-normal);
}

.form-group input:focus,
.form-group select:focus {
  outline: none;
  border-color: var(--primary);
  box-shadow: 0 0 0 3px var(--color-primary-alpha);
}

.form-group small {
  display: block;
  margin-top: 0.5rem;
  color: var(--text-tertiary);
  font-size: var(--font-size-sm);
}

.checkbox-group {
  display: flex;
  align-items: flex-start;
  gap: 0.75rem;
  margin-bottom: 1rem;
}

.checkbox-group input[type="checkbox"] {
  width: auto;
  margin-top: 0.25rem;
}

.checkbox-group label {
  flex: 1;
  margin: 0;
  font-weight: var(--font-weight-normal);
}
`;

interface WizardStep {
  number: number;
  label: string;
  icon: React.ReactNode;
}

const steps: WizardStep[] = [
  { number: 1, label: 'Domain', icon: <Globe size={20} /> },
  { number: 2, label: 'Complete', icon: <Check size={20} /> },
];

interface InitializationWizardProps {
  onComplete?: () => void;
}

const InitializationWizard: React.FC<InitializationWizardProps> = ({ onComplete }) => {
  const navigate = useNavigate();
  const [currentStep, setCurrentStep] = useState(1);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [completed, setCompleted] = useState(false);

  // Form data
  const [domainName, setDomainName] = useState('');

  const canProceed = () => {
    switch (currentStep) {
      case 1:
        return domainName.trim().length > 0;
      default:
        return true;
    }
  };

  const handleNext = () => {
    if (canProceed() && currentStep < steps.length) {
      setCurrentStep(currentStep + 1);
      setError(null);
    }
  };

  const handleBack = () => {
    if (currentStep > 1) {
      setCurrentStep(currentStep - 1);
      setError(null);
    }
  };

  const handleFinish = async () => {
    setLoading(true);
    setError(null);

    try {
      // Step 1: Initialize user (create default workspace and user settings)
      console.log('Initializing user account...');
      const initResponse = await apiService.user.initialize();
      console.log('User initialization completed:', initResponse.message);

      // Step 2: Create the domain
      console.log('Creating domain:', domainName);
      const domainData: CreateDomainDto = {
        name: domainName,
      };

      await apiService.domains.create(domainData);
      console.log('Domain created successfully');

      setCompleted(true);
      setLoading(false);

      // Notify parent that setup is complete
      if (onComplete) {
        onComplete();
      }

      // Redirect to dashboard after 2 seconds
      setTimeout(() => {
        navigate('/');
      }, 2000);
    } catch (err: any) {
      console.error('Initialization failed:', err);
      setError(err.response?.data?.message || err.message || 'Failed to complete initialization. Please try again.');
      setLoading(false);
    }
  };

  const renderStepContent = () => {
    if (completed) {
      return (
        <div className="wizard-success">
          <CheckCircle className="wizard-success-icon" size={80} />
          <h2>Setup Complete!</h2>
          <p>Your account has been initialized successfully. Redirecting to dashboard...</p>
        </div>
      );
    }

    switch (currentStep) {
      case 1:
        return (
          <div className="wizard-step-content">
            <h2 className="wizard-step-title">
              <Globe size={24} />
              Configure Your Domain
            </h2>
            <p className="wizard-step-description">
              Enter your primary domain name. This will be used for creating short links.
            </p>

            <div className="form-group">
              <label htmlFor="domain-name">Domain Name *</label>
              <input
                id="domain-name"
                type="text"
                value={domainName}
                onChange={(e) => setDomainName(e.target.value)}
                placeholder="example.com"
                autoFocus
              />
              <small>Enter your domain without http:// or https://</small>
            </div>

            {error && (
              <div className="alert alert-error" style={{ marginTop: '1rem' }}>
                {error}
              </div>
            )}
          </div>
        );

      case 2:
        return (
          <div className="wizard-step-content">
            <h2 className="wizard-step-title">
              <Check size={24} />
              Review & Complete
            </h2>
            <p className="wizard-step-description">
              Review your settings and complete the setup.
            </p>

            <div className="card" style={{ padding: '1.5rem', marginBottom: '1.5rem' }}>
              <h3 style={{ margin: '0 0 1rem 0', fontSize: 'var(--font-size-md)', fontWeight: 'var(--font-weight-semibold)', color: 'var(--text-primary)' }}>Configuration Summary</h3>

              <div style={{ display: 'grid', gap: '0.75rem' }}>
                <div>
                  <strong style={{ color: 'var(--text-secondary)', fontSize: 'var(--font-size-sm)' }}>Domain:</strong>
                  <div style={{ marginTop: '0.25rem', color: 'var(--text-primary)' }}>{domainName}</div>
                </div>
              </div>
            </div>

            {error && (
              <div className="alert alert-error">
                {error}
              </div>
            )}
          </div>
        );

      default:
        return null;
    }
  };

  return (
    <>
      <style>{wizardStyles}</style>
      <div className="wizard-container">
        <div className="wizard-card">
          <div className="wizard-header">
            <h1>
              <Rocket size={32} style={{ display: 'inline-block', verticalAlign: 'middle', marginRight: '0.5rem' }} />
              Welcome to Shortas
            </h1>
            <p>Let's get your account set up in just a few steps</p>
          </div>

          <div className="wizard-progress">
            {steps.map((step, index) => (
              <React.Fragment key={step.number}>
                <div className={`wizard-step ${currentStep === step.number ? 'active' : ''} ${currentStep > step.number ? 'completed' : ''}`}>
                  <div className="wizard-step-number">
                    {currentStep > step.number ? <Check size={20} /> : step.number}
                  </div>
                  <span className="wizard-step-label">{step.label}</span>
                </div>
                {index < steps.length - 1 && <div className="wizard-divider" />}
              </React.Fragment>
            ))}
          </div>

          <div className="wizard-body">
            {loading ? (
              <LoadingSpinner />
            ) : (
              renderStepContent()
            )}
          </div>

          {!completed && !loading && (
            <div className="wizard-footer">
              <button
                className="btn btn-outline"
                onClick={handleBack}
                disabled={currentStep === 1}
              >
                <ChevronLeft size={20} />
                Back
              </button>

              {currentStep < steps.length ? (
                <button
                  className="btn btn-primary"
                  onClick={handleNext}
                  disabled={!canProceed()}
                >
                  Next
                  <ChevronRight size={20} />
                </button>
              ) : (
                <button
                  className="btn btn-primary"
                  onClick={handleFinish}
                  disabled={loading}
                >
                  <Check size={20} />
                  Complete Setup
                </button>
              )}
            </div>
          )}
        </div>
      </div>
    </>
  );
};

export default InitializationWizard;
