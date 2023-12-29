// Detects when a click happens outside of the specified element and calls a callback
export function clickOutside(element: HTMLElement, cb: (e: MouseEvent) => void) {
  const handleClick = (event: MouseEvent): void => {
    if (!element.contains(event.target as Node)) {
      cb(event);
    }
  }
  document.body.addEventListener("click", handleClick);
  return {
    destroy(): void {
      document.body.removeEventListener("click", handleClick);
    },
  };
}
