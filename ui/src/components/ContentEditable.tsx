import { useEffect, useRef, useState } from 'react'

interface ContentEditableProps {
  onChange: (e: React.ChangeEvent<HTMLDivElement>) => void
  className?: string
  value?: string
  onRefTarget?: (ref: HTMLDivElement) => void
}

type Props = Readonly<ContentEditableProps> &
  React.HTMLAttributes<HTMLDivElement>

export default function ContentEditable({
  value,
  className,
  onChange,
  onRefTarget,
  ...props
}: Props): JSX.Element {
  const [timeout, setTimeoutState] = useState<NodeJS.Timeout>()
  const ref = useRef<HTMLDivElement>(null)

  // TODO: fix cursor jumping to beginning of text when saving

  useEffect(() => {
    if (ref.current !== null && onRefTarget !== undefined) {
      onRefTarget(ref.current)
    }
  }, [ref.current])

  useEffect(() => {
    if (ref.current !== null && value !== undefined) {
      ref.current.innerText = value
    }
  }, [value])

  const onInput = (e: React.ChangeEvent<HTMLDivElement>): void => {
    if (timeout !== undefined) {
      clearTimeout(timeout)
    }

    // debounce
    setTimeoutState(
      setTimeout(() => {
        onChange(e)
      }, 1000),
    )
  }

  return (
    <div
      {...props}
      ref={ref}
      className={className}
      contentEditable
      onInput={onInput}
      suppressContentEditableWarning
    >
      {value}
    </div>
  )
}
