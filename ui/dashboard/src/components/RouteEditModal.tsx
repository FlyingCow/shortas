import React, { useState, useEffect } from 'react';
import { 
  X, 
  Plus, 
  Trash2, 
  Save,
  Globe,
  Smartphone,
  Monitor,
  Clock,
  MapPin,
  User,
  Settings
} from 'lucide-react';
import { Modal, Form, Button, Row, Col, Card, Badge, Alert } from 'react-bootstrap';
import { RouteDto } from '../services/api';
import './DesignSystem.css';

interface ConditionalRule {
  id: string;
  condition: 'device' | 'location' | 'time' | 'user_agent' | 'referrer' | 'ip';
  operator: 'equals' | 'contains' | 'starts_with' | 'ends_with' | 'regex' | 'in_range' | 'not_equals';
  value: string;
  destination: string;
  priority: number;
}

interface RouteEditModalProps {
  show: boolean;
  onHide: () => void;
  route?: RouteDto | null;
  onSave: (routeData: any) => void;
}

const RouteEditModal: React.FC<RouteEditModalProps> = ({ show, onHide, route, onSave }) => {
  const [formData, setFormData] = useState({
    link: '',
    destination: '',
    status: 'active',
    code: 302,
    ttl: 3600,
    description: ''
  });

  const [conditionalRules, setConditionalRules] = useState<ConditionalRule[]>([]);
  const [showRuleBuilder, setShowRuleBuilder] = useState(false);
  const [newRule, setNewRule] = useState<Partial<ConditionalRule>>({
    condition: 'device',
    operator: 'equals',
    value: '',
    destination: '',
    priority: 1
  });

  useEffect(() => {
    if (route) {
      setFormData({
        link: route.link || '',
        destination: route.dest || '',
        status: route.status || 'active',
        code: route.code || 302,
        ttl: route.ttl || 3600,
        description: (route as any).description || ''
      });
    } else {
      setFormData({
        link: '',
        destination: '',
        status: 'active',
        code: 302,
        ttl: 3600,
        description: ''
      });
    }
  }, [route]);

  const handleInputChange = (field: string, value: any) => {
    setFormData(prev => ({ ...prev, [field]: value }));
  };

  const addConditionalRule = () => {
    if (newRule.condition && newRule.operator && newRule.value && newRule.destination) {
      const rule: ConditionalRule = {
        id: `rule_${Date.now()}`,
        condition: newRule.condition as any,
        operator: newRule.operator as any,
        value: newRule.value,
        destination: newRule.destination,
        priority: newRule.priority || 1
      };
      
      setConditionalRules(prev => [...prev, rule].sort((a, b) => a.priority - b.priority));
      setNewRule({
        condition: 'device',
        operator: 'equals',
        value: '',
        destination: '',
        priority: conditionalRules.length + 2
      });
      setShowRuleBuilder(false);
    }
  };

  const removeRule = (ruleId: string) => {
    setConditionalRules(prev => prev.filter(rule => rule.id !== ruleId));
  };

  const updateRulePriority = (ruleId: string, priority: number) => {
    setConditionalRules(prev => 
      prev.map(rule => 
        rule.id === ruleId ? { ...rule, priority } : rule
      ).sort((a, b) => a.priority - b.priority)
    );
  };

  const handleSave = () => {
    const routeData = {
      ...formData,
      conditionalRules: conditionalRules.length > 0 ? conditionalRules : undefined
    };
    onSave(routeData);
    onHide();
  };

  const getConditionIcon = (condition: string) => {
    switch (condition) {
      case 'device': return <Smartphone size={16} />;
      case 'location': return <MapPin size={16} />;
      case 'time': return <Clock size={16} />;
      case 'user_agent': return <Monitor size={16} />;
      case 'referrer': return <Globe size={16} />;
      case 'ip': return <User size={16} />;
      default: return <Settings size={16} />;
    }
  };

  const getOperatorLabel = (operator: string) => {
    switch (operator) {
      case 'equals': return 'equals';
      case 'contains': return 'contains';
      case 'starts_with': return 'starts with';
      case 'ends_with': return 'ends with';
      case 'regex': return 'matches regex';
      case 'in_range': return 'in range';
      case 'not_equals': return 'not equals';
      default: return operator;
    }
  };

  return (
    <Modal show={show} onHide={onHide} size="lg" centered className="route-edit-modal">
      <Modal.Header closeButton>
        <Modal.Title>
          {route ? 'Edit Route' : 'Create New Route'}
        </Modal.Title>
      </Modal.Header>
      
      <Modal.Body style={{ maxHeight: '70vh', overflowY: 'auto' }}>
        <Form>
          {/* Basic Route Information */}
          <Card className="mb-3">
            <Card.Header>
              <h5 className="mb-0">Basic Information</h5>
            </Card.Header>
            <Card.Body>
              <Row>
                <Col md={6}>
                  <Form.Group className="mb-3">
                    <Form.Label>Short URL</Form.Label>
                    <Form.Control
                      type="text"
                      value={formData.link}
                      onChange={(e) => handleInputChange('link', e.target.value)}
                      placeholder="e.g., my-link"
                    />
                  </Form.Group>
                </Col>
                <Col md={6}>
                  <Form.Group className="mb-3">
                    <Form.Label>Default Destination</Form.Label>
                    <Form.Control
                      type="url"
                      value={formData.destination}
                      onChange={(e) => handleInputChange('destination', e.target.value)}
                      placeholder="https://example.com"
                    />
                  </Form.Group>
                </Col>
              </Row>
              
              <Row>
                <Col md={4}>
                  <Form.Group className="mb-3">
                    <Form.Label>Status</Form.Label>
                    <Form.Select
                      value={formData.status}
                      onChange={(e) => handleInputChange('status', e.target.value)}
                    >
                      <option value="active">Active</option>
                      <option value="inactive">Inactive</option>
                    </Form.Select>
                  </Form.Group>
                </Col>
                <Col md={4}>
                  <Form.Group className="mb-3">
                    <Form.Label>HTTP Code</Form.Label>
                    <Form.Select
                      value={formData.code}
                      onChange={(e) => handleInputChange('code', parseInt(e.target.value))}
                    >
                      <option value={301}>301 - Permanent Redirect</option>
                      <option value={302}>302 - Temporary Redirect</option>
                      <option value={307}>307 - Temporary Redirect (Preserve Method)</option>
                      <option value={308}>308 - Permanent Redirect (Preserve Method)</option>
                    </Form.Select>
                  </Form.Group>
                </Col>
                <Col md={4}>
                  <Form.Group className="mb-3">
                    <Form.Label>TTL (seconds)</Form.Label>
                    <Form.Control
                      type="number"
                      value={formData.ttl}
                      onChange={(e) => handleInputChange('ttl', parseInt(e.target.value))}
                      min="60"
                      max="86400"
                    />
                  </Form.Group>
                </Col>
              </Row>
              
              <Form.Group className="mb-3">
                <Form.Label>Description</Form.Label>
                <Form.Control
                  as="textarea"
                  rows={2}
                  value={formData.description}
                  onChange={(e) => handleInputChange('description', e.target.value)}
                  placeholder="Optional description for this route"
                />
              </Form.Group>
            </Card.Body>
          </Card>

          {/* Conditional Routing Rules */}
          <Card className="mb-3">
            <Card.Header className="d-flex justify-content-between align-items-center">
              <h5 className="mb-0">Conditional Routing Rules</h5>
              <Button
                variant="outline-primary"
                size="sm"
                onClick={() => setShowRuleBuilder(true)}
              >
                <Plus size={16} />
                Add Rule
              </Button>
            </Card.Header>
            <Card.Body>
              {conditionalRules.length === 0 ? (
                <Alert variant="info" className="mb-0">
                  <strong>No conditional rules set.</strong> This route will always redirect to the default destination.
                  Click "Add Rule" to create conditional redirects based on device, location, time, or other criteria.
                </Alert>
              ) : (
                <div className="space-y-2">
                  {conditionalRules.map((rule, index) => (
                    <div key={rule.id} className="rule-item d-flex align-items-center justify-content-between">
                      <div className="d-flex align-items-center">
                        <div className="rule-icon">
                          {getConditionIcon(rule.condition)}
                        </div>
                        <div className="rule-content">
                          <div className="rule-title">
                            {rule.condition.charAt(0).toUpperCase() + rule.condition.slice(1)} {getOperatorLabel(rule.operator)} "{rule.value}"
                          </div>
                          <div className="rule-subtitle">
                            → {rule.destination}
                          </div>
                        </div>
                      </div>
                      <div className="rule-actions">
                        <Badge bg="secondary" className="me-2">
                          Priority {rule.priority}
                        </Badge>
                        <Button
                          variant="outline-danger"
                          size="sm"
                          onClick={() => removeRule(rule.id)}
                        >
                          <Trash2 size={14} />
                        </Button>
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </Card.Body>
          </Card>

          {/* Rule Builder */}
          {showRuleBuilder && (
            <Card className="mb-3">
              <Card.Header className="d-flex justify-content-between align-items-center">
                <h5 className="mb-0">Add New Rule</h5>
                <Button
                  variant="outline-secondary"
                  size="sm"
                  onClick={() => setShowRuleBuilder(false)}
                >
                  <X size={16} />
                </Button>
              </Card.Header>
              <Card.Body>
                <Row>
                  <Col md={4}>
                    <Form.Group className="mb-3">
                      <Form.Label>Condition Type</Form.Label>
                      <Form.Select
                        value={newRule.condition}
                        onChange={(e) => setNewRule(prev => ({ ...prev, condition: e.target.value as any }))}
                      >
                        <option value="device">Device Type</option>
                        <option value="location">Location (Country)</option>
                        <option value="time">Time of Day</option>
                        <option value="user_agent">User Agent</option>
                        <option value="referrer">Referrer</option>
                        <option value="ip">IP Address</option>
                      </Form.Select>
                    </Form.Group>
                  </Col>
                  <Col md={4}>
                    <Form.Group className="mb-3">
                      <Form.Label>Operator</Form.Label>
                      <Form.Select
                        value={newRule.operator}
                        onChange={(e) => setNewRule(prev => ({ ...prev, operator: e.target.value as any }))}
                      >
                        <option value="equals">Equals</option>
                        <option value="contains">Contains</option>
                        <option value="starts_with">Starts With</option>
                        <option value="ends_with">Ends With</option>
                        <option value="regex">Regex Match</option>
                        <option value="not_equals">Not Equals</option>
                      </Form.Select>
                    </Form.Group>
                  </Col>
                  <Col md={4}>
                    <Form.Group className="mb-3">
                      <Form.Label>Priority</Form.Label>
                      <Form.Control
                        type="number"
                        value={newRule.priority || 1}
                        onChange={(e) => setNewRule(prev => ({ ...prev, priority: parseInt(e.target.value) }))}
                        min="1"
                        max="100"
                      />
                    </Form.Group>
                  </Col>
                </Row>
                
                <Row>
                  <Col md={6}>
                    <Form.Group className="mb-3">
                      <Form.Label>Value</Form.Label>
                      <Form.Control
                        type="text"
                        value={newRule.value || ''}
                        onChange={(e) => setNewRule(prev => ({ ...prev, value: e.target.value }))}
                        placeholder={
                          newRule.condition === 'device' ? 'mobile, desktop, tablet' :
                          newRule.condition === 'location' ? 'US, GB, DE, FR' :
                          newRule.condition === 'time' ? '09:00-17:00' :
                          newRule.condition === 'user_agent' ? 'Chrome, Firefox, Safari' :
                          newRule.condition === 'referrer' ? 'google.com, facebook.com' :
                          newRule.condition === 'ip' ? '192.168.1.0/24' :
                          'Enter value'
                        }
                      />
                    </Form.Group>
                  </Col>
                  <Col md={6}>
                    <Form.Group className="mb-3">
                      <Form.Label>Destination URL</Form.Label>
                      <Form.Control
                        type="url"
                        value={newRule.destination || ''}
                        onChange={(e) => setNewRule(prev => ({ ...prev, destination: e.target.value }))}
                        placeholder="https://example.com/special-page"
                      />
                    </Form.Group>
                  </Col>
                </Row>
                
                <div className="d-flex justify-content-end">
                  <Button
                    variant="outline-secondary"
                    className="me-2"
                    onClick={() => setShowRuleBuilder(false)}
                  >
                    Cancel
                  </Button>
                  <Button
                    variant="primary"
                    onClick={addConditionalRule}
                    disabled={!newRule.condition || !newRule.operator || !newRule.value || !newRule.destination}
                  >
                    Add Rule
                  </Button>
                </div>
              </Card.Body>
            </Card>
          )}
        </Form>
      </Modal.Body>
      
      <Modal.Footer>
        <Button variant="outline-secondary" onClick={onHide}>
          Cancel
        </Button>
        <Button variant="primary" onClick={handleSave}>
          <Save size={16} className="me-1" />
          {route ? 'Update Route' : 'Create Route'}
        </Button>
      </Modal.Footer>
    </Modal>
  );
};

export default RouteEditModal;
