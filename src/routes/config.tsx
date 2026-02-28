import { createFileRoute } from '@tanstack/react-router';
import { invoke } from '@tauri-apps/api/core';
import { SaveIcon } from 'lucide-react';
import { useEffect, useState } from 'react';

import { Button } from '@/components/ui/button';
import { Textarea } from '@/components/ui/textarea';

export const Route = createFileRoute('/config')({
  component: Index,
});

function Index() {
  const [config, setConfig] = useState('');

  useEffect(() => {
    invoke<string>('get_frpc_config').then(setConfig);
  }, []);

  return (
    <div className="flex h-full flex-col gap-2 p-2">
      <Textarea
        className="grow font-mono"
        value={config}
        onChange={(e) => setConfig(e.target.value)}
      />
      <Button onClick={() => invoke('set_frpc_config', { config })}>
        <SaveIcon />
        Save
      </Button>
    </div>
  );
}
