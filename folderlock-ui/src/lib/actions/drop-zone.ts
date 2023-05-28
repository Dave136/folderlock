/** Dispatch actions on all drop & drag events */
export function dropZone(node: HTMLElement) {
  // actions states / hover / enter / etc
  const dragEnter = (event: DragEvent) => {
    event.preventDefault();
    node.dispatchEvent(
      new CustomEvent('dragEnter', { detail: { isOverDropZone: true } }),
    );
  };

  const dragLeave = (event: DragEvent) => {
    event.preventDefault();
    node.dispatchEvent(
      new CustomEvent('dragLeave', { detail: { isOverDropZone: false } }),
    );
  };

  const dragOver = (event: DragEvent) => {};
  const drop = (event: DragEvent) => {};

  // attach events listeners to the node
  node.addEventListener('dragenter', dragEnter, false);
  node.addEventListener('dragover', dragOver, false);
  node.addEventListener('dragleave', dragLeave, false);
  node.addEventListener('drop', drop, false);

  // destroy any existing listeners
  return {
    destroy() {
      node.removeEventListener('dragenter', dragEnter);
      node.removeEventListener('dragover', dragOver);
      node.removeEventListener('dragleave', dragLeave);
      node.removeEventListener('drop', drop);
    },
  };
}
