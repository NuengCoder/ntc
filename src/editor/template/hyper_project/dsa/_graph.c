#include "_graph.h"
#include <stdlib.h>
#include <string.h>

Graph* graph_create(int initial_capacity, int directed) {
Graph* g = (Graph*)malloc(sizeof(Graph));
if (!g) return NULL;
if (initial_capacity <= 0) initial_capacity = 16;
g->vertices = (GVertex*)calloc(initial_capacity, sizeof(GVertex));
if (!g->vertices) { free(g); return NULL; }
g->vertex_count = 0;
g->vertex_capacity = initial_capacity;
g->directed = directed;
return g;
}

void graph_destroy(Graph* g, void (*free_data)(void*)) {
if (!g) return;
for (int i = 0; i < g->vertex_count; i++) {
free(g->vertices[i].edges);
if (free_data) free_data(g->vertices[i].data);
}
free(g->vertices);
free(g);
}

int graph_add_vertex(Graph* g, void* data) {
if (!g) return -1;
if (g->vertex_count >= g->vertex_capacity) {
int nc = g->vertex_capacity * 2;
GVertex* nv = (GVertex*)realloc(g->vertices, sizeof(GVertex) * nc);
if (!nv) return -1;
memset(&nv[g->vertex_count], 0, sizeof(GVertex) * (nc - g->vertex_count));
g->vertices = nv;
g->vertex_capacity = nc;
}
int id = g->vertex_count++;
g->vertices[id].id = id;
g->vertices[id].data = data;
g->vertices[id].edges = NULL;
g->vertices[id].edge_count = 0;
g->vertices[id].edge_capacity = 0;
return id;
}

int graph_add_edge(Graph* g, int from, int to, int weight) {
if (!g || from < 0 || from >= g->vertex_count || to < 0 || to >= g->vertex_count)
return -1;
GVertex* v = &g->vertices[from];
if (v->edge_count >= v->edge_capacity) {
int nc = v->edge_capacity == 0 ? 4 : v->edge_capacity * 2;
GEdge* ne = (GEdge*)realloc(v->edges, sizeof(GEdge) * nc);
if (!ne) return -1;
v->edges = ne;
v->edge_capacity = nc;
}
v->edges[v->edge_count].dest_id = to;
v->edges[v->edge_count].weight = weight;
v->edge_count++;
if (!g->directed && from != to)
return graph_add_edge(g, to, from, weight);
return 0;
}

static void dfs_rec(Graph* g, int v, int* visited, void (*visit)(int, void*)) {
visited[v] = 1;
visit(v, g->vertices[v].data);
for (int i = 0; i < g->vertices[v].edge_count; i++) {
int d = g->vertices[v].edges[i].dest_id;
if (!visited[d]) dfs_rec(g, d, visited, visit);
}
}

void graph_bfs(Graph* g, int start, void (*visit)(int, void*)) {
if (!g || start < 0 || start >= g->vertex_count || !visit) return;
int* visited = (int*)calloc(g->vertex_count, sizeof(int));
if (!visited) return;
int* q = (int*)malloc(sizeof(int) * g->vertex_count);
if (!q) { free(visited); return; }
int f = 0, b = 0;
visited[start] = 1;
q[b++] = start;
while (f < b) {
int v = q[f++];
visit(v, g->vertices[v].data);
for (int i = 0; i < g->vertices[v].edge_count; i++) {
int d = g->vertices[v].edges[i].dest_id;
if (!visited[d]) { visited[d] = 1; q[b++] = d; }
}
}
free(q);
free(visited);
}

void graph_dfs(Graph* g, int start, void (*visit)(int, void*)) {
if (!g || start < 0 || start >= g->vertex_count || !visit) return;
int* visited = (int*)calloc(g->vertex_count, sizeof(int));
if (!visited) return;
dfs_rec(g, start, visited, visit);
free(visited);
}

int graph_vertex_count(Graph* g) { return g ? g->vertex_count : 0; }