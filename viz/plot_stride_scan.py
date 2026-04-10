#!/usr/bin/env python3
"""
Aletheia Stride Scan Visualization Script

Reads JSONL experiment results and creates a plot showing runtime vs stride
for CPU vs memory_engine modes, demonstrating the effect of cache locality
on execution performance.
"""

import json
import pandas as pd
import matplotlib.pyplot as plt
from pathlib import Path
import sys


def load_results(filepath):
    """Load JSONL experiment results into a pandas DataFrame."""
    results = []
    try:
        with open(filepath, 'r') as f:
            for line in f:
                if line.strip():
                    results.append(json.loads(line))
        return pd.DataFrame(results)
    except FileNotFoundError:
        print(f"Error: Results file not found: {filepath}")
        sys.exit(1)
    except json.JSONDecodeError as e:
        print(f"Error: Invalid JSON in results file: {e}")
        sys.exit(1)


def plot_stride_scan(df, output_dir):
    """
    Plot runtime vs stride for stride scan.
    Shows both CPU and memory_engine modes on the same graph.
    """
    # Filter data for stride_scan experiment
    stride_data = df[df['experiment'] == 'stride_scan'].copy()
    
    if stride_data.empty:
        print("Warning: No stride_scan data found")
        return
    
    # Sort by stride
    stride_data = stride_data.sort_values('stride')
    
    # Create figure and axis
    fig, ax = plt.subplots(figsize=(12, 7))
    
    # Plot CPU mode
    cpu_data = stride_data[stride_data['mode'] == 'cpu']
    if not cpu_data.empty:
        ax.plot(
            cpu_data['stride'],
            cpu_data['runtime_ms'],
            marker='o',
            linewidth=2.5,
            markersize=10,
            label='CPU Mode',
            color='#FF6B6B'
        )
    
    # Plot memory_engine mode
    mem_data = stride_data[stride_data['mode'] == 'memory_engine']
    if not mem_data.empty:
        ax.plot(
            mem_data['stride'],
            mem_data['runtime_ms'],
            marker='s',
            linewidth=2.5,
            markersize=10,
            label='Memory Engine Mode',
            color='#4ECDC4'
        )
    
    # Configure axes and labels
    ax.set_xlabel('Stride Value', fontsize=13, fontweight='bold')
    ax.set_ylabel('Runtime (ms)', fontsize=13, fontweight='bold')
    ax.set_title('Stride Scan - Effect of Memory Locality on Runtime', 
                 fontsize=15, fontweight='bold', pad=20)
    
    # Add grid
    ax.grid(True, alpha=0.3, linestyle='--')
    
    # Configure legend
    ax.legend(loc='upper left', fontsize=12, framealpha=0.95)
    
    # Format x-axis to show stride values
    strides = stride_data['stride'].unique()
    ax.set_xticks(sorted(strides))
    
    # Add annotation explaining the pattern
    ax.text(0.98, 0.02, 
            'Higher stride → larger memory jumps → worse cache locality → increased runtime',
            transform=ax.transAxes,
            fontsize=10,
            verticalalignment='bottom',
            horizontalalignment='right',
            bbox=dict(boxstyle='round', facecolor='wheat', alpha=0.3))
    
    # Tight layout
    plt.tight_layout()
    
    # Create output directory if it doesn't exist
    output_path = Path(output_dir) / 'stride_scan.png'
    output_path.parent.mkdir(parents=True, exist_ok=True)
    
    # Save figure
    plt.savefig(output_path, dpi=300, bbox_inches='tight')
    print(f"✓ Saved: {output_path}")
    
    plt.close()


def main():
    """Main entry point."""
    results_file = Path('results/stride_scan.jsonl')
    output_dir = Path('viz/output')
    
    if not results_file.exists():
        print(f"Error: Results file not found: {results_file}")
        sys.exit(1)
    
    print(f"Loading stride scan results from {results_file}")
    df = load_results(results_file)
    
    if df.empty:
        print("Error: No data loaded from results file")
        sys.exit(1)
    
    print(f"Loaded {len(df)} records")
    print(f"Experiments: {df['experiment'].unique()}")
    print(f"Modes: {df['mode'].unique()}")
    print(f"Strides: {sorted(df['stride'].unique())}")
    
    plot_stride_scan(df, output_dir)


if __name__ == '__main__':
    main()
