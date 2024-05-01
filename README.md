# Simple DAG

## Objective

You are provided with a list of nodes that have been reduced to:

- their unique identifier: id
- two nodes that they reference:left and right

Your task is to parse this list into a Directed Acyclic Graph and provide statistics such as:

- Average depth of the DAG (assume that the graph has a single point of origin ID=1 and depth is the shortest path to it)
- Average number of nodes per depth (not including depth 0)
- Average number of in-references per node
- Be creative, find some more statistics that you think would be interesting to have

## Input & Output

In addition to being able to manually create a DAG, add nodes and set references, you should be able to instantiate a DAG from a plain text file, with a structure as follows:

- Line 1: N, the number of nodes in the database
- Lines 2 through N + 1: the node data, where each node consists of the ids of its left and right parents
- Node id 1 is the unique origin of all nodes
- The id of each node in the database is its line number

```
# Database template

1  N      # integer, number of nodes
2  L R    # integers describing a node, L and R = Left and Right parent node ids
3  . .    #
4  . .
5  . .
```

The graph produced is as following:

![alt text](/public/upload_a572430baeb681c3dad73b835cddb198.png)

The program is expected to be run and output to the console as follows:

```
$ ./dag database.txt
> AVG DAG DEPTH: ???
> AVG NODES PER DEPTH: ???
> AVG IN-REF: ???
> <YOUR STAT>: ???
> <YOUR STAT>: ???
> <YOUR STAT>: ???
...
```

## Example

database.txt

```
5
1 1
1 2
2 2
3 6
3 3
```

Running and output:

```
$ ./dag database.txt
> AVG DAG DEPTH: 1.33
> AVG NODES PER DEPTH: 2.5
> AVG REF: 1.667
```

## Solution

For the solution, I used both DFS and BFS algorithms to showcase that there are several algorithms to solve graph related problems.

- `DFS` algorithm for cycle check

  DFS allows us to explore the graph by visiting each node and its neighbors in a systematic way. In a DAG, if we encounter a back edge while performing DFS, it indicates the presence of a cycle. This is because in a directed graph, a back edge from a node to one of its parents in the DFS tree represents a cycle. Therefore, by performing DFS and detecting back edges, we can determine whether the graph contains a cycle or not.

- `BFS` algorithm for calculating depth of each nodes in DAG

  BFS systematically explores the graph level by level, starting from a given source node. In a DAG, where there are no cycles, BFS allows us to traverse the graph layer by layer, assigning depths to each node from the source node. Since a DAG has a topological ordering, BFS can efficiently compute the depths of each node by visiting nodes in a specific order and updating their depths traversing the graph. This makes BFS an effective algorithm for computing the depths of nodes in a DAG.

## Run & Test

- If you have installed Rust locally, you can run using the following command:

  ```bash
  cargo build
  cargo run
  ```

  You can run the tests using the following command:

  ```bash
  cargo test
  ```

- If you have not installed Rust locally, you can run using the follwoing command:

  To build docker image for this application, run the following command in the root directory of the project:

  ```bash
  docker build -t simple_dag .
  ```

  Once the docker image built, you can run the application as a Docker container using the following command:

  ```bash
  docker run simple_dag
  ```

## Documentation

To see more detailed information for each functions, please use the following command:

```bash
cargo doc
```

Then please visit the url: `target/doc/simple_dag/index.html`
