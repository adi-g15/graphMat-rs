#pragma once

/**
 * This header is not meant to be included directly
*/

#include "../../util/coord.hpp"
#include <map>
#include <vector>
#include <algorithm>
#include <memory>

#include "box_base.hpp"

class Matrix_Base {

protected:
	const int _dimension;	// 2,3,...

	int _get_matrix_dimension() { return this->_dimension; }

	static constexpr float init_expansion_speed{ 1 }; // @future - allow negative growths too
	void auto_expansion();
	void pause_auto_expansion();
	void resume_auto_expansion();

	Matrix_Base(int dimension) : _dimension(dimension) {}
	~Matrix_Base() {}

	// virtual ~Matrix_Base();
};

template<class graph_box_type>
class MemoryAlloc {
private:
	// @q -> If i allocated using `new Graph_Box_3D<int>[10]` and deleteing using `delete[] BaseClass*` will it free all the occupied memory earlier ? It should, if it internally uses malloc or free
	//inline static std::map<Matrix_Base*, std::vector<Box_Base*>> allocated_memory_loc;	// error C2243: 'type cast': conversion from 'Graph_Box_3D<int> *' to 'Box_Base*' exists, but is inaccessible
	std::vector<graph_box_type*> allocated_memory_loc;
	//std::vector<std::unique_ptr<graph_box_type>> allocated_memory_loc;

public:
	//static T* Alloc(Matrix_Base* mat_ptr, int num = 1) {
	graph_box_type* Alloc(int num = 1) {
		//if (num < 1)	throw std::invalid_argument("Can't allocate 0 or negative numbers of blocks in MemoryAlloc::Alloc");
		if (num < 1)	return nullptr;
		static int total = 0;
		total += num;

#ifdef LOG_ALLOCATION
		std::fprintf(stderr, "[Total - %d KB] Allocating %d bytes\n", total / 1000, num);
		std::fflush(stderr);
#endif // DEBUG

		this->allocated_memory_loc
			.emplace_back(new graph_box_type[num]);

		return allocated_memory_loc.back();	// NOT USING `new` and `delete` for now, since VS debug builds will give you debug assertion failed, since we are freeing memory with `delete` instead of `delete[]`, which it accepts as better, since we allocated with `new[]`, but i don't wan't to leave our advantage either 
	}

	//static void DeAlloc(Matrix_Base* mat_ptr) {
	void DeAllocAll() {
		//std::for_each(allocated_memory_loc.begin(), allocated_memory_loc.end(), operator delete[]);
		for (auto* mem_ptr : allocated_memory_loc)
		{
			delete[] mem_ptr;
		}
	}

	~MemoryAlloc() { this->DeAllocAll(); }
};
