import Vector from './Vector';

export default function VectorSet(props: { vectors: number[][] }) {
  return (
    <div class="flex flex-wrap gap-1.5 items-center">
      {props.vectors.slice(0, 4).map((vec, i) => (
        <Vector vector={vec} />
      ))}
      {props.vectors.length > 4 ? (
        <span class="text-foreground">...</span>
      ) : null}
    </div>
  );
}
