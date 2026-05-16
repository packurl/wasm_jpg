/* tslint:disable */
/* eslint-disable */
declare module 'jpg' {
  /**
   * Encodes the supplied ImageData rgba array.
   * @param {Uint8Array} bytes
   * @param {number} width
   * @param {number} height
   * @param {number} [quality=80] (1 to 100)
   * @return {Uint8Array}
   */
  export function jpg(bytes: Uint8Array, width: number, height: number, quality: number): Uint8Array;

  export default jpg;
}
