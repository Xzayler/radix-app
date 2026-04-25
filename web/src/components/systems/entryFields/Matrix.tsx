import katex from 'katex';
import { createEffect, createSignal, ErrorBoundary } from 'solid-js';

function generateKatexString(matrix: number[][], max?: number): string {
  const toShow = max ?? 4;
  let latexString;
  if (matrix.length > toShow) {
    const lastRow = new Array(toShow + 1).fill('...').join(' & ');
    latexString = matrix
      .slice(0, toShow)
      .map((row) => row.slice(0, toShow).join(' & ') + ' & ...')
      .concat(lastRow)
      .join(' \\\\ ');
  } else {
    latexString = matrix.map((row) => row.join(' & ')).join(' \\\\ ');
  }

  return katex.renderToString(
    `\\begin{bmatrix} ${latexString} \\end{bmatrix}`,
    {
      throwOnError: false,
      displayMode: true,
    },
  );
}

export default function BaseMatrix(props: {
  matrix: number[][];
  toShow?: number;
}) {
  const [katexHtml, setKatexHtml] = createSignal<string>(
    generateKatexString(props.matrix, props.toShow),
  );

  createEffect(() => {
    setKatexHtml(generateKatexString(props.matrix, props.toShow));
  });

  return <div class="text-foreground" innerHTML={katexHtml()}></div>;
}
