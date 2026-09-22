/** Shared decoration only: this component never loads the chart engine. */
export function CelestialBackdrop() {
  return (
    <div className="celestial-backdrop" aria-hidden="true">
      <div className="celestial-nebula" />
      <div className="celestial-veil" />
    </div>
  );
}
