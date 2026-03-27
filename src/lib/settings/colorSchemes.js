export const colorSchemes = {
  dark: {
    background: '#000000',
    foreground: '#ffffff',
    cursor: '#ffffff',
    black: '#000000',
    red: '#cc0000',
    green: '#088907',
    yellow: '#dfb600',
    blue: '#0000aa',
    magenta: '#671673',
    cyan: '#00797b',
    white: '#d3d7cf',
    brightBlack: '#555753',
    brightRed: '#ff0000',
    brightGreen: '#00dd00',
    brightYellow: '#ffff00',
    brightBlue: '#0000ff',
    brightMagenta: '#e430cf',
    brightCyan: '#04c7c7',
    brightWhite: '#ffffff',
  },
  classic: {
    background: '#1e1e1e',
    foreground: '#d4d4d4',
    cursor: '#d4d4d4',
    black: '#1e1e1e',
    red: '#f44747',
    green: '#6a9955',
    yellow: '#dcdcaa',
    blue: '#569cd6',
    magenta: '#c586c0',
    cyan: '#4ec9b0',
    white: '#d4d4d4',
    brightBlack: '#808080',
    brightRed: '#f44747',
    brightGreen: '#6a9955',
    brightYellow: '#dcdcaa',
    brightBlue: '#569cd6',
    brightMagenta: '#c586c0',
    brightCyan: '#4ec9b0',
    brightWhite: '#ffffff',
  },
};

/** @param {string} schemeName */
export function getTheme(schemeName) {
  return colorSchemes[/** @type {keyof colorSchemes} */ (schemeName)] || colorSchemes.dark;
}
