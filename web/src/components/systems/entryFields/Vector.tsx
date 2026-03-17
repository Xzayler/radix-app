import katex from 'katex';

export default function Vector(props: { vector: number[] }) {
  const html = katex.renderToString(
    `\\begin{bmatrix} ${props.vector.join(' \\\\ ')} \\end{bmatrix}`,
    {
      throwOnError: false,
    },
  );
  return <div class="text-foreground" innerHTML={html}></div>;
}
