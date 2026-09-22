/** Armillary-inspired ornament, independent of calculated stars and palaces. */
export function CelestialOrbits() {
  return (
    <svg
      className="celestial-orbits"
      viewBox="0 0 1000 640"
      fill="none"
      aria-hidden="true"
      focusable="false"
    >
      <g className="celestial-orbit">
        <ellipse
          cx="500"
          cy="320"
          rx="445"
          ry="162"
          transform="rotate(-27 500 320)"
        />
        <circle cx="500" cy="320" r="302" />
        <path d="M80 102L113 76L158 94L183 64L224 100" />
        <path d="M805 496L835 455L875 475L905 430" />
      </g>
      <g className="celestial-orbit celestial-orbit-cool">
        <ellipse
          cx="500"
          cy="320"
          rx="438"
          ry="165"
          transform="rotate(28 500 320)"
        />
        <circle cx="500" cy="320" r="310" strokeDasharray="1 12" />
      </g>
      <g className="celestial-stars">
        {[
          [80, 102],
          [113, 76],
          [158, 94],
          [183, 64],
          [224, 100],
          [805, 496],
          [835, 455],
          [875, 475],
          [905, 430],
        ].map(([cx, cy]) => (
          <circle key={`${cx}-${cy}`} cx={cx} cy={cy} r="1.7" />
        ))}
        <circle cx="116" cy="468" r="3" fill="none" />
        <circle cx="885" cy="173" r="3" fill="none" />
      </g>
    </svg>
  );
}
