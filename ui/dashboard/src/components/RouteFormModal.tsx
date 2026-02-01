import React, { useState, useEffect } from 'react';
import { X } from 'lucide-react';
import { RouteDto, DomainDto, apiService } from '../services/api';
import RouteForm from './RouteForm';
import './DesignSystem.css';

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
      <div className="modal-content" onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <h2 className="modal-title">
            {route ? 'Edit Route' : 'Create New Route'}
          </h2>
          <button className="modal-close" onClick={onClose}>
            <X size={20} />
          </button>
        </div>

        <div className="modal-body">
          <RouteForm
            route={route}
            domains={domains}
            onSave={onSave}
            onCancel={onClose}
          />
        </div>
      </div>
    </div>
  );
};

export default RouteFormModal;
