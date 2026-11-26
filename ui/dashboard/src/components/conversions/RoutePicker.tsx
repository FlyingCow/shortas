import React, { useEffect, useState } from 'react';
import { apiService, RouteDto } from '../../services/api';

const PAGE_SIZE = 50;

interface Props {
  workspaceId: string;
  selected: string[];
  onChange: (ids: string[]) => void;
}

const RoutePicker: React.FC<Props> = ({ workspaceId, selected, onChange }) => {
  const [search, setSearch] = useState('');
  const [routes, setRoutes] = useState<RouteDto[]>([]);
  const [total, setTotal] = useState(0);
  const [error, setError] = useState(false);

  useEffect(() => {
    let current = true;
    const timer = setTimeout(async () => {
      try {
        const page = await apiService.routes.list({
          workspaceId,
          page: 1,
          pageSize: PAGE_SIZE,
          search: search.trim() || undefined,
        });
        if (!current) return;
        setRoutes(page.data);
        setTotal(page.pagination.totalCount);
        setError(false);
      } catch {
        if (current) setError(true);
      }
    }, 250);
    return () => {
      current = false;
      clearTimeout(timer);
    };
  }, [workspaceId, search]);

  const toggle = (id: string) =>
    onChange(selected.includes(id) ? selected.filter((s) => s !== id) : [...selected, id]);

  return (
    <div>
      <input
        className="form-control"
        aria-label="Search routes"
        placeholder="Search routes..."
        value={search}
        onChange={(e) => setSearch(e.target.value)}
      />
      <div className="cv-routes">
        {error && <span className="cv-error">Failed to load routes.</span>}
        {routes.map((r) => (
          <label key={r.id} className="cv-check">
            <input type="checkbox" checked={selected.includes(r.id!)} onChange={() => toggle(r.id!)} />
            {r.domain?.name ? `${r.domain.name}/${r.link}` : r.link}
          </label>
        ))}
        {!error && routes.length === 0 && <span className="cv-help">No routes found.</span>}
      </div>
      <span className="cv-help">
        {selected.length} selected.
        {total > routes.length && ` Showing ${routes.length} of ${total}; search to find others.`}
      </span>
    </div>
  );
};

export default RoutePicker;
