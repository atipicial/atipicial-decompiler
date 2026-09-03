export class AtipicialDecompilerError extends Error {
  constructor(message, details = {}) {
    super(message);
    this.name = this.constructor.name;
    this.details = details;
  }
}

export class AefParseError extends AtipicialDecompilerError {}

export class DisassemblyError extends AtipicialDecompilerError {}

export class ManifestParseError extends AtipicialDecompilerError {}
