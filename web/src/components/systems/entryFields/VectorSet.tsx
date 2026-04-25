import { For } from 'solid-js';
import Vector from './Vector';

export default function VectorSet(props: {
  vectors: number[][];
  showAll?: boolean;
  toShow?: number;
}) {
  const toShow = props.toShow ?? 4;
  return (
    <div class="flex gap-1.5 items-center">
      <For
        each={props.vectors.slice(
          0,
          props.showAll ? props.vectors.length : toShow,
        )}
      >
        {(vec) => {
          return <Vector vector={vec} />;
        }}
      </For>
      {props.vectors.length > toShow && !props.showAll ? (
        <span class="text-foreground">...</span>
      ) : null}
    </div>
  );
}
