vm_stat | awk '
  BEGIN { 
    print "Memory Statistics (MB):"; 
    page_size = 16384; # Page size in bytes, as reported
  }
  /Pages free:/ { 
    free_pages = $NF; 
    gsub("[^0-9]", "", free_pages); 
    printf "Free Memory:         %.2f MB\n", free_pages * page_size / 1024 / 1024; 
  }
  /Pages active:/ { 
    active_pages = $NF; 
    gsub("[^0-9]", "", active_pages); 
    printf "Active Memory:       %.2f MB\n", active_pages * page_size / 1024 / 1024; 
  }
  /Pages inactive:/ { 
    inactive_pages = $NF; 
    gsub("[^0-9]", "", inactive_pages); 
    printf "Inactive Memory:     %.2f MB\n", inactive_pages * page_size / 1024 / 1024; 
  }
  /Pages wired down:/ { 
    wired_pages = $NF; 
    gsub("[^0-9]", "", wired_pages); 
    printf "Wired Memory:        %.2f MB\n", wired_pages * page_size / 1024 / 1024; 
  }
  /Pages occupied by compressor:/ { 
    compressed_pages = $NF; 
    gsub("[^0-9]", "", compressed_pages); 
    printf "Compressed Memory:   %.2f MB\n", compressed_pages * page_size / 1024 / 1024; 
  }
'

