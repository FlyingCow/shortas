import React from 'react';
import { ConversionStatus } from '../../services/api';

const CLASSES: Record<ConversionStatus, string> = {
  Draft: 'badge-secondary',
  Activating: 'badge-warning',
  Active: 'badge-success',
  Paused: 'badge-warning',
  Archived: 'badge-secondary',
};

const StatusBadge: React.FC<{ status: ConversionStatus }> = ({ status }) => (
  <span className={`badge ${CLASSES[status]}`}>{status}</span>
);

export default StatusBadge;
