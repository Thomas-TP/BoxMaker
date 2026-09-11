import { X } from "lucide-react";
import { type ReactNode, useEffect, useRef } from "react";

export function Modal({
  open,
  onClose,
  title,
  children,
}: {
  open: boolean;
  onClose: () => void;
  title: string;
  children: ReactNode;
}) {
  const ref = useRef<HTMLDialogElement>(null);
  useEffect(() => {
    const dialog = ref.current;
    if (open && !dialog?.open) dialog?.showModal();
    if (!open && dialog?.open) dialog.close();
  }, [open]);
  return (
    <dialog
      ref={ref}
      className="app-dialog"
      aria-label={title}
      onCancel={onClose}
      onClose={onClose}
    >
      <button
        type="button"
        className="icon-button modal-close"
        onClick={onClose}
        aria-label="Fermer la fenêtre"
      >
        <X size={20} />
      </button>
      {children}
    </dialog>
  );
}
