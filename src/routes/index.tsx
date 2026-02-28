import { createFileRoute } from '@tanstack/react-router';
import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { List, useListRef } from 'react-window';
import { useInterval } from 'usehooks-ts';

import { LogRow } from '@/components/log-row';

export const Route = createFileRoute('/')({
  component: RouteComponent,
});

function RouteComponent() {
  const [logs, setLogs] = useState<string[]>([]);
  const listRef = useListRef(null);

  const loadLogs = () => {
    invoke<string[]>('get_frpc_logs').then(setLogs);
  };
  useEffect(loadLogs, []);
  useInterval(loadLogs, 1000);
  useEffect(() => {
    if (listRef.current && logs.length > 0) {
      listRef.current.scrollToRow({ index: logs.length - 1 });
    }
  }, [listRef, logs]);

  return (
    <div className="h-full p-2">
      <List
        listRef={listRef}
        className="h-full"
        rowComponent={LogRow}
        rowCount={logs.length}
        rowHeight={24}
        rowProps={{ logs }}
      />
    </div>
  );
}
