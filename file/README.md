# Read-only file

## Question description

File is an important concept in OS. In this question, you need to implement a simple read-only file using `Vec<u8>` which includes the following function:

1. The file has construct and read function.
2. Each file has its owner. Once a file is created, its owner cannot be changed.
3. The file has a permission check and can update the permitted users. The owner is not included in the permitted users.
4. The file will update the latest read time when a read operation is success.

## Evaluation

Check your answer with `eval.sh`
