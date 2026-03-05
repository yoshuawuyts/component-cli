# Validation

r[validation.success]
Validation MUST pass when manifest and lockfile are consistent.

r[validation.missing-dependency]
Validation MUST detect packages in the lockfile that are not in the manifest.

r[validation.invalid-dependency]
Validation MUST detect package dependencies referencing non-existent packages.

r[validation.empty]
Validation MUST pass for empty manifest and lockfile pairs.

r[validation.error-display]
Validation errors MUST have human-readable display messages.

r[validation.mixed-types]
Validation MUST handle both component and interface sections.

r[validation.invalid-version-constraint]
Validation MUST detect and report invalid semver version constraints in
manifest dependencies.

r[validation.version-conflict]
Validation MUST detect conflicting version constraints for the same package
name across components and interfaces sections.
