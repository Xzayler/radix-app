export const template = `{
  "name": "My System",  // Remove comments for proper parsing
  "dimension": 2,       // Required, the dimension of the system.
  "base": [             // A square matrix, array of arrays of integers.
    [2, -1],            //  Number of rows and columns must match
    [1, 2]              //  the dimension.
  ],
  "digits": {           // Required object
    "type": "Explicit", // Required, type of digits. Can be Explicit, Canonical, JCanonical, Adjoined, Symmetric, JSymmetric, Shifted
    "jValue": 1,        // Required for JCanonical, JSymetric digits. Do not include otherwise
    "shift": 1,         // Required for Shifted digits. A positive integer. Must be <= |det(base)|
    "values": [         // Required for Explicit digits. It is an array of arrays of integers
      [0, 0],           // Each inner array must be of length = dimension
      [1, 0],
      [0, 1],
      [0, -1],
      [-6, 5]
    ]
  }
}`;
