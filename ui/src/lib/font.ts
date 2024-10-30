// https://www.w3.org/Style/Examples/007/fonts.en.html

const fonts = [
    "Arial, sans-serif",
    "Helvetica, sans-serif",
    "Verdana, sans-serif",
    "Trebuchet MS, sans-serif",
    "Gill Sans, sans-serif",
    "Noto Sans, sans-serif",
    "Avantgarde, TeX Gyre Adventor, URW Gothic L, sans-serif",
    "Optima, sans-serif",
    "Arial Narrow, sans-serif",
    "Times, Times New Roman, serif",
    "Didot, serif",
    "Georgia, serif",
    "Palatino, URW Palladio L, serif",
    "Bookman, URW Bookman L, serif",
    "New Century Schoolbook, TeX Gyre Schola, serif",
    "American Typewriter, serif",
    "Andale Mono, monospace",
    "Courier New, monospace",
    "Courier, monospace",
    "FreeMono, monospace",
    "OCR A Std, monospace",
    "DejaVu Sans Mono, monospace",
    "Comic Sans MS, Comic Sans, cursive",
    "Apple Chancery, cursive",
    "Bradley Hand, cursive",
    "Brush Script MT, Brush Script Std, cursive",
    "Snell Roundhand, cursive",
    "URW Chancery L, cursive",
    "Impact, fantasy",
    "Luminari, fantasy",
    "Chalkduster, fantasy",
    "Jazz LET, fantasy",
    "Blippo, fantasy",
    "Stencil Std, fantasy",
    "Marker Felt, fantasy",
    "Trattatello, fantasy",
];

const stretches = [
    "ultra-condensed",
    "extra-condensed",
    "condensed",
    "semi-condensed",
    "normal",
    "semi-expanded",
    "expanded",
    "extra-expanded",
    "ultra-expanded",
];

const styles = ["normal", "italic", "oblique"];
const weights = ["100", "200", "300", "normal", "500", "600", "bold", "800", "900"];
const variants = ["normal", "small-caps"];

const rand = <T>(arr: T[]) => arr[Math.floor(Math.random() * arr.length)];
const rands = <T>(arr: T[], num: number): T[] =>
    Array(num)
        .fill(() => rand(arr))
        .map((v) => v());

export const randFont = () => rand(fonts);
export const randStretch = () => rand(stretches);
export const randStyle = () => rand(styles);
export const randWeight = () => rand(weights);
export const randVariant = () => rand(variants);

export const randFonts = (num: number) => rands(fonts, num);
export const randStretches = (num: number) => rands(stretches, num);
export const randStyles = (num: number) => rands(styles, num);
export const randWeights = (num: number) => rands(weights, num);
export const randVariants = (num: number) => rands(variants, num);
