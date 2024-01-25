import { useEffect, useRef, useState } from "react";

interface ContentEditableProps {
  onChange: (e: React.ChangeEvent<HTMLDivElement>) => void;
  className?: string;
  value?: string;
}

export default function ContentEditable({ value, className, onChange }: Readonly<ContentEditableProps>): JSX.Element {
  const ref = useRef<HTMLDivElement>(null);
  const [timeout, setTimeoutState] = useState<NodeJS.Timeout>();

  // TODO: fix cursor jumping to beginning of text when saving

  useEffect(() => {
    if (ref.current && value) {
      ref.current.innerText = value;
    }
  }, [value]);

  const onInput = (e: React.ChangeEvent<HTMLDivElement>) => {
    if (timeout) {
      clearTimeout(timeout);
    }

    // debounce
    setTimeoutState(
      setTimeout(() => {
        onChange(e);
      }, 1000)
    );
  };

  return (
    <div ref={ref} className={className} contentEditable onInput={onInput} suppressContentEditableWarning>
      {value}
    </div>
  );
}
