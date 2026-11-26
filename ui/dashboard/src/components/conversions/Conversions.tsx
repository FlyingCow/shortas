import React from 'react';
import { Navigate, Route, Routes } from 'react-router-dom';
import { useWorkspace } from '../../contexts/WorkspaceContext';
import LoadingSpinner from '../LoadingSpinner';
import ConversionDetail from './ConversionDetail';
import ConversionEditor from './ConversionEditor';
import ConversionList from './ConversionList';
import './Conversions.css';

const Conversions: React.FC = () => {
  const { workspaces, loading, currentWorkspaceId } = useWorkspace();
  const workspace = workspaces.find((w) => w.id === currentWorkspaceId);

  if (!workspace) {
    return loading ? (
      <LoadingSpinner message="Loading workspace..." />
    ) : (
      <div className="empty-state">
        <div className="empty-state-title">No workspace</div>
        <div className="empty-state-description">Conversions belong to a workspace. Create one first.</div>
      </div>
    );
  }

  // The API enforces this; here it only hides what would be refused.
  const canManage = workspace.userRole !== 'Member';
  const props = { workspaceId: workspace.id, canManage };
  // Keyed, so switching workspaces never shows the previous one's data.
  return (
    <Routes key={workspace.id}>
      <Route index element={<ConversionList {...props} />} />
      <Route path="new" element={canManage ? <ConversionEditor {...props} /> : <Navigate to="/conversions" replace />} />
      <Route path=":id" element={<ConversionDetail {...props} />} />
      <Route path=":id/edit" element={canManage ? <ConversionEditor {...props} /> : <Navigate to="/conversions" replace />} />
      <Route path="*" element={<Navigate to="/conversions" replace />} />
    </Routes>
  );
};

export interface ConversionPageProps {
  workspaceId: string;
  canManage: boolean;
}

export default Conversions;
