import type { RowComponentProps } from 'react-window';

const getColor = (log: string) => {
  switch (log[25]) {
    case 'I':
      return 'text-blue-500';
    case 'W':
      return 'text-yellow-500';
    case 'E':
      return 'text-red-500';
    default:
      return '';
  }
};

export function LogRow({
  index,
  logs,
  style,
  ariaAttributes,
}: RowComponentProps<{ logs: string[] }>) {
  const log = logs[index];
  const color = getColor(log);

  return (
    <p className={`line-clamp-1 font-mono ${color}`} style={style} {...ariaAttributes}>
      {log}
    </p>
  );
}
