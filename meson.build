project(
    '16bit',
    'cpp',
    default_options : [
        'optimization=3',
        'cpp_std=c++2a',
        'warning_level=3',
        'werror=true'
    ])

main_incdir = include_directories('src')

src = files([
  'src/cpu/cpu.cpp',
  'src/memory/memory.cpp',
  'src/memory/writablememory.cpp',
  'src/memory/memorymapper.cpp',
  'src/devices/screendevice.cpp'
])

fmt = subproject('fmt').get_variable('fmt_dep')
plog = subproject('sergiusthebest-plog').get_variable('plog_dep')

_exe = executable('16bit',
  sources: ['src/main.cpp', src],
  include_directories: main_incdir,
  dependencies: [fmt, plog])

#-- tests
tests_incdir = include_directories('tests/include')

memory_tests = executable('memory_tests',
  sources: ['tests/memory.cpp', src],
  include_directories: [tests_incdir, main_incdir],
  dependencies: [fmt, plog])
test('memory tests', memory_tests)

writable_memory_tests = executable('writable_memory_tests',
  sources: ['tests/writablememory.cpp', src],
  include_directories: [tests_incdir, main_incdir],
  dependencies: [fmt, plog])
test('writable memory tests', writable_memory_tests)

cpu_tests = executable('cpu_tests',
  sources: ['tests/cpu.cpp', src],
  include_directories: [tests_incdir, main_incdir],
  dependencies: [fmt, plog])
test('cpu tests', cpu_tests)
#---
