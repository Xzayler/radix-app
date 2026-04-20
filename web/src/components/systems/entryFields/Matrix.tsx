import katex from 'katex';

export default function BaseMatrix(props: {
  matrix: number[][];
  toShow?: number;
}) {
  const toShow = props.toShow ?? 4;
  let latexString;
  if (props.matrix.length > toShow) {
    const lastRow = new Array(toShow + 1).fill('...').join(' & ');
    latexString = props.matrix
      .slice(0, toShow)
      .map((row) => row.slice(0, toShow).join(' & ') + ' & ...')
      .concat(lastRow)
      .join(' \\\\ ');
  } else {
    latexString = props.matrix.map((row) => row.join(' & ')).join(' \\\\ ');
  }

  const html = katex.renderToString(
    `\\begin{bmatrix} ${latexString} \\end{bmatrix}`,
    {
      throwOnError: false,
      displayMode: true,
    },
  );

  return <div class="text-foreground" innerHTML={html}></div>;
}
