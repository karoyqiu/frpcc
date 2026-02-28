import { createFileRoute } from '@tanstack/react-router';
import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { List } from 'react-window';
import { useInterval } from 'usehooks-ts';

import { LogRow } from '@/components/log-row';

export const Route = createFileRoute('/')({
  component: RouteComponent,
});

function RouteComponent() {
  const [logs, setLogs] = useState<string[]>([]);

  const loadLogs = () => {
    invoke<string[]>('get_frpc_logs').then(setLogs);
  };

  useEffect(loadLogs, []);

  useInterval(loadLogs, 1000);

  return (
    <div className="h-full p-2">
      <List
        className="h-full"
        rowComponent={LogRow}
        rowCount={logs.length}
        rowHeight={24}
        rowProps={{ logs }}
      />
    </div>
  );
}
