declare namespace JSX {
    interface IntrinsicElements {
        'midi-player': React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement> & {
            src?: string;
            visualizer?: string;
        };
        'midi-visualizer': React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement> & {
            type?: string;
            id?: string;
        };
    }
}