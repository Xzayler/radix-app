import Vector from './Vector';

export default function VectorSet(props: {
  vectors: number[][];
  toShow?: number;
}) {
  const toShow = props.toShow ?? 4;
  return (
    <div class="flex gap-1.5 items-center">
      {props.vectors.slice(0, toShow).map((vec, i) => (
        <Vector vector={vec} />
      ))}
      {props.vectors.length > toShow ? (
        <span class="text-foreground">...</span>
      ) : null}
    </div>
  );
}
