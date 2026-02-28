import { createFileRoute } from '@tanstack/react-router';
import { useState } from 'react';

export const Route = createFileRoute('/log')({
  component: RouteComponent,
});

function RouteComponent() {
  const [logs, setLogs] = useState<string[]>([]);
  return <div>Hello "/log"!</div>;
}
