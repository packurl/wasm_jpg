/* tslint:disable */
/* eslint-disable */
declare module 'jpg_worker' {
  /**
   * Encodes the supplied ImageData rgba array.
   * @param {Uint8Array} bytes
   * @param {number} width
   * @param {number} height
   * @param {number} [quality=80] (1 to 100)
   * @param {boolean} [transfer=false]
   * @return {Promise<Uint8Array>}
   */
  export function jpg(
    bytes: Uint8Array, width: number, height: number, quality: number, transfer: boolean
  ): Promise<Uint8Array>;

  export default jpg;
}