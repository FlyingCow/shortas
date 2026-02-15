import React, { useState, useEffect } from 'react';
import { RouteDto, DomainDto, apiService } from '../services/api';
import RouteForm from './RouteForm';
import './DesignSystem.css';
import './RouteForm.css';

interface RouteFormModalProps {
  show: boolean;
  onClose: () => void;
  onSave: (route: Partial<RouteDto>) => Promise<void>;
  route?: RouteDto | null;
}

export const RouteFormModal: React.FC<RouteFormModalProps> = ({
  show,
  onClose,
  onSave,
  route,
}) => {
  const [domains, setDomains] = useState<DomainDto[]>([]);

  useEffect(() => {
    if (show) {
      fetchDomains();
    }
  }, [show]);

  const fetchDomains = async () => {
    try {
      const response = await apiService.domains.list({ page: 1, pageSize: 100 });
      setDomains(response.data);
    } catch (err) {
      console.error('Failed to fetch domains:', err);
    }
  };

  if (!show) return null;

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal-content modal-content-no-padding" onClick={(e) => e.stopPropagation()}>
        <RouteForm
          route={route}
          domains={domains}
          onSave={onSave}
          onCancel={onClose}
        />
      </div>
    </div>
  );
};

export default RouteFormModal;
