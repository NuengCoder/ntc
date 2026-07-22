typedef struct GVertex {
    int id;
    void* data;
    struct GEdge* edges;
    int edge_count;
    int edge_capacity;
} GVertex;

typedef struct GEdge {
    int dest_id;
    int weight;
} GEdge;

typedef struct Graph {
    GVertex* vertices;
    int vertex_count;
    int vertex_capacity;
    int directed;
} Graph;

Graph* graph_create(int initial_capacity, int directed);
void   graph_destroy(Graph* g, void (*free_data)(void*));
int    graph_add_vertex(Graph* g, void* data);
int    graph_add_edge(Graph* g, int from, int to, int weight);
void   graph_bfs(Graph* g, int start, void (*visit)(int, void*));
void   graph_dfs(Graph* g, int start, void (*visit)(int, void*));
int    graph_vertex_count(Graph* g);